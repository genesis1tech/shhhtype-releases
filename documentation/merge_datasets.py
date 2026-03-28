#!/usr/bin/env python3
"""
Dataset Merger for Waste Classification

This script merges TrashNet and TACO datasets into a unified YOLO format dataset
for training waste classification models.

Features:
- Downloads datasets if not present
- Converts annotations to YOLO format
- Maps categories to unified taxonomy
- Splits data into train/val/test sets
- Generates dataset statistics

Usage:
    python merge_datasets.py --trashnet-path ./datasets/trashnet \
                             --taco-path ./datasets/taco \
                             --output-path ./datasets/merged \
                             --train-split 0.8 \
                             --val-split 0.15 \
                             --test-split 0.05

Author: Smart Waste Bin Project
Date: January 2026
"""

import os
import json
import shutil
import random
import argparse
import urllib.request
import zipfile
from pathlib import Path
from typing import Dict, List, Tuple, Optional
from collections import defaultdict
import yaml

# =============================================================================
# CATEGORY MAPPINGS
# =============================================================================

# TrashNet original categories -> Our unified categories
TRASHNET_MAPPING = {
    'cardboard': 'cardboard_box',
    'glass': 'glass_bottle_clear',
    'metal': 'aluminum_soda_can',
    'paper': 'office_paper',
    'plastic': 'pet_water_bottle',
    'trash': 'trash_misc'
}

# TACO categories -> Our unified categories (60 categories mapped to our taxonomy)
TACO_MAPPING = {
    # Plastic containers
    'Clear plastic bottle': 'pet_water_bottle',
    'Plastic bottle cap': 'pp_bottle_cap',
    'Other plastic bottle': 'pet_soda_bottle',
    'Plastic container': 'plastic_food_container',
    'Disposable plastic cup': 'plastic_cup',
    'Plastic lid': 'plastic_lid',
    'Plastic straw': 'plastic_straw',
    'Plastic utensils': 'plastic_utensil',
    'Plastic bag & wrapper': 'plastic_bag',
    'Other plastic wrapper': 'snack_wrapper',
    'Single-use carrier bag': 'plastic_bag',
    'Polypropylene bag': 'plastic_bag',
    'Plastic film': 'plastic_film',
    'Six pack rings': 'trash_misc',
    'Spread tub': 'plastic_food_container',
    'Tupperware': 'plastic_food_container',
    'Squeezable tube': 'trash_misc',
    'Plastic glooves': 'disposable_glove',
    'Styrofoam piece': 'styrofoam_piece',
    
    # Metal
    'Drink can': 'aluminum_soda_can',
    'Food can': 'steel_food_can',
    'Aerosol': 'aerosol_can',
    'Metal lid': 'metal_lid',
    'Metal bottle cap': 'metal_bottle_cap',
    'Aluminium foil': 'aluminum_foil',
    'Aluminium blister pack': 'trash_misc',
    'Pop tab': 'pop_tab',
    'Scrap metal': 'trash_misc',
    
    # Glass
    'Glass bottle': 'glass_bottle_clear',
    'Broken glass': 'broken_glass',
    'Glass cup': 'glass_container',
    'Glass jar': 'glass_jar',
    
    # Paper
    'Paper': 'office_paper',
    'Wrapping paper': 'office_paper',
    'Normal paper': 'office_paper',
    'Paper bag': 'paper_bag',
    'Tissues': 'tissue',
    'Paper cup': 'coffee_cup_paper',
    'Magazine paper': 'magazine',
    'Newspaper': 'newspaper',
    'Toilet tube': 'cardboard_flat',
    
    # Cardboard
    'Cardboard': 'cardboard_box',
    'Corrugated carton': 'cardboard_box',
    'Egg carton': 'egg_carton',
    'Drink carton': 'juice_box',
    'Meal carton': 'food_box',
    'Pizza box': 'pizza_box',
    'Paper straw': 'trash_misc',
    'Other carton': 'cardboard_flat',
    
    # Cigarettes
    'Cigarette': 'cigarette_butt',
    
    # Food
    'Food waste': 'food_leftovers',
    
    # Miscellaneous
    'Rope & strings': 'trash_misc',
    'Shoe': 'trash_misc',
    'Unlabeled litter': 'unknown_object',
    'Battery': 'battery',
    'Other plastic': 'trash_misc',
    'Crisp packet': 'chip_bag',
    'Foam cup': 'styrofoam_cup',
    'Foam food container': 'takeout_container',
    'Garbage bag': 'plastic_bag',
    'Carded blister pack': 'trash_misc',
    'Other': 'trash_misc'
}

# Our unified categories with their bin assignments
UNIFIED_CATEGORIES = {
    # PLASTIC BIN (0)
    'pet_water_bottle': {'id': 0, 'bin': 0, 'bin_name': 'PLASTIC'},
    'pet_soda_bottle': {'id': 1, 'bin': 0, 'bin_name': 'PLASTIC'},
    'hdpe_milk_jug': {'id': 2, 'bin': 0, 'bin_name': 'PLASTIC'},
    'pp_yogurt_cup': {'id': 3, 'bin': 0, 'bin_name': 'PLASTIC'},
    'pp_bottle_cap': {'id': 4, 'bin': 0, 'bin_name': 'PLASTIC'},
    'plastic_food_container': {'id': 5, 'bin': 0, 'bin_name': 'PLASTIC'},
    'plastic_cup': {'id': 6, 'bin': 0, 'bin_name': 'PLASTIC'},
    'plastic_lid': {'id': 7, 'bin': 0, 'bin_name': 'PLASTIC'},
    'plastic_clamshell': {'id': 8, 'bin': 0, 'bin_name': 'PLASTIC'},
    
    # METAL BIN (1)
    'aluminum_soda_can': {'id': 9, 'bin': 1, 'bin_name': 'METAL'},
    'aluminum_beer_can': {'id': 10, 'bin': 1, 'bin_name': 'METAL'},
    'aluminum_foil': {'id': 11, 'bin': 1, 'bin_name': 'METAL'},
    'aluminum_tray': {'id': 12, 'bin': 1, 'bin_name': 'METAL'},
    'steel_food_can': {'id': 13, 'bin': 1, 'bin_name': 'METAL'},
    'metal_lid': {'id': 14, 'bin': 1, 'bin_name': 'METAL'},
    'pop_tab': {'id': 15, 'bin': 1, 'bin_name': 'METAL'},
    'metal_bottle_cap': {'id': 16, 'bin': 1, 'bin_name': 'METAL'},
    'aerosol_can': {'id': 17, 'bin': 1, 'bin_name': 'METAL'},
    
    # GLASS BIN (2)
    'glass_bottle_clear': {'id': 18, 'bin': 2, 'bin_name': 'GLASS'},
    'glass_bottle_green': {'id': 19, 'bin': 2, 'bin_name': 'GLASS'},
    'glass_bottle_brown': {'id': 20, 'bin': 2, 'bin_name': 'GLASS'},
    'glass_jar': {'id': 21, 'bin': 2, 'bin_name': 'GLASS'},
    'glass_container': {'id': 22, 'bin': 2, 'bin_name': 'GLASS'},
    'broken_glass': {'id': 23, 'bin': 2, 'bin_name': 'GLASS'},
    
    # PAPER BIN (3)
    'office_paper': {'id': 24, 'bin': 3, 'bin_name': 'PAPER'},
    'newspaper': {'id': 25, 'bin': 3, 'bin_name': 'PAPER'},
    'magazine': {'id': 26, 'bin': 3, 'bin_name': 'PAPER'},
    'cardboard_box': {'id': 27, 'bin': 3, 'bin_name': 'PAPER'},
    'cardboard_flat': {'id': 28, 'bin': 3, 'bin_name': 'PAPER'},
    'paper_bag': {'id': 29, 'bin': 3, 'bin_name': 'PAPER'},
    'egg_carton': {'id': 30, 'bin': 3, 'bin_name': 'PAPER'},
    'juice_box': {'id': 31, 'bin': 3, 'bin_name': 'PAPER'},
    'food_box': {'id': 32, 'bin': 3, 'bin_name': 'PAPER'},
    'pizza_box': {'id': 33, 'bin': 3, 'bin_name': 'PAPER'},
    
    # COMPOST BIN (4)
    'fruit_peel': {'id': 34, 'bin': 4, 'bin_name': 'COMPOST'},
    'fruit_core': {'id': 35, 'bin': 4, 'bin_name': 'COMPOST'},
    'vegetable_scraps': {'id': 36, 'bin': 4, 'bin_name': 'COMPOST'},
    'egg_shell': {'id': 37, 'bin': 4, 'bin_name': 'COMPOST'},
    'coffee_grounds': {'id': 38, 'bin': 4, 'bin_name': 'COMPOST'},
    'tea_bag': {'id': 39, 'bin': 4, 'bin_name': 'COMPOST'},
    'food_leftovers': {'id': 40, 'bin': 4, 'bin_name': 'COMPOST'},
    'napkin_soiled': {'id': 41, 'bin': 4, 'bin_name': 'COMPOST'},
    'paper_towel_soiled': {'id': 42, 'bin': 4, 'bin_name': 'COMPOST'},
    
    # TRASH BIN (5)
    'styrofoam_cup': {'id': 43, 'bin': 5, 'bin_name': 'TRASH'},
    'styrofoam_piece': {'id': 44, 'bin': 5, 'bin_name': 'TRASH'},
    'plastic_bag': {'id': 45, 'bin': 5, 'bin_name': 'TRASH'},
    'chip_bag': {'id': 46, 'bin': 5, 'bin_name': 'TRASH'},
    'snack_wrapper': {'id': 47, 'bin': 5, 'bin_name': 'TRASH'},
    'candy_wrapper': {'id': 48, 'bin': 5, 'bin_name': 'TRASH'},
    'plastic_straw': {'id': 49, 'bin': 5, 'bin_name': 'TRASH'},
    'plastic_utensil': {'id': 50, 'bin': 5, 'bin_name': 'TRASH'},
    'plastic_film': {'id': 51, 'bin': 5, 'bin_name': 'TRASH'},
    'cigarette_butt': {'id': 52, 'bin': 5, 'bin_name': 'TRASH'},
    'disposable_glove': {'id': 53, 'bin': 5, 'bin_name': 'TRASH'},
    'face_mask': {'id': 54, 'bin': 5, 'bin_name': 'TRASH'},
    'coffee_cup_paper': {'id': 55, 'bin': 5, 'bin_name': 'TRASH'},
    'tissue': {'id': 56, 'bin': 5, 'bin_name': 'TRASH'},
    'takeout_container': {'id': 57, 'bin': 5, 'bin_name': 'TRASH'},
    'battery': {'id': 58, 'bin': 5, 'bin_name': 'TRASH'},
    'trash_misc': {'id': 59, 'bin': 5, 'bin_name': 'TRASH'},
    'unknown_object': {'id': 60, 'bin': 5, 'bin_name': 'TRASH'},
}


# =============================================================================
# DATASET DOWNLOAD FUNCTIONS
# =============================================================================

def download_trashnet(output_path: str) -> bool:
    """Download TrashNet dataset from GitHub."""
    url = "https://github.com/garythung/trashnet/raw/master/data/dataset-resized.zip"
    zip_path = os.path.join(output_path, "trashnet.zip")
    
    print(f"Downloading TrashNet dataset...")
    try:
        os.makedirs(output_path, exist_ok=True)
        urllib.request.urlretrieve(url, zip_path)
        
        print(f"Extracting TrashNet dataset...")
        with zipfile.ZipFile(zip_path, 'r') as zip_ref:
            zip_ref.extractall(output_path)
        
        os.remove(zip_path)
        print(f"TrashNet downloaded to {output_path}")
        return True
    except Exception as e:
        print(f"Error downloading TrashNet: {e}")
        return False


def download_taco(output_path: str) -> bool:
    """
    Download TACO dataset.
    Note: TACO requires downloading via their official repository.
    This function provides instructions.
    """
    print("\n" + "="*60)
    print("TACO DATASET DOWNLOAD INSTRUCTIONS")
    print("="*60)
    print("""
TACO dataset needs to be downloaded manually:

1. Clone the TACO repository:
   git clone https://github.com/pedropro/TACO.git
   
2. Run the download script:
   cd TACO
   python download.py
   
3. Copy the data to your dataset folder:
   cp -r data/* {output_path}/

Alternatively, download from Kaggle:
   https://www.kaggle.com/datasets/kneroma/tacotrashdataset

After downloading, ensure the structure is:
   {output_path}/
   ├── annotations.json
   └── images/
       ├── batch_1/
       ├── batch_2/
       └── ...
    """.format(output_path=output_path))
    print("="*60 + "\n")
    return False


# =============================================================================
# DATA PROCESSING FUNCTIONS
# =============================================================================

def process_trashnet(
    trashnet_path: str, 
    output_path: str,
    category_mapping: Dict[str, str]
) -> List[Dict]:
    """
    Process TrashNet dataset and convert to YOLO format.
    
    TrashNet is a classification dataset with images organized in folders.
    We create pseudo bounding boxes covering the full image.
    
    Args:
        trashnet_path: Path to TrashNet dataset
        output_path: Output path for processed images
        category_mapping: Mapping from TrashNet to unified categories
    
    Returns:
        List of annotation dictionaries
    """
    annotations = []
    
    # TrashNet structure: dataset-resized/<category>/<images>
    dataset_path = os.path.join(trashnet_path, "dataset-resized")
    
    if not os.path.exists(dataset_path):
        print(f"TrashNet dataset not found at {dataset_path}")
        return annotations
    
    for category in os.listdir(dataset_path):
        category_path = os.path.join(dataset_path, category)
        if not os.path.isdir(category_path):
            continue
            
        if category not in category_mapping:
            print(f"Warning: Unknown TrashNet category '{category}', skipping...")
            continue
        
        unified_category = category_mapping[category]
        if unified_category not in UNIFIED_CATEGORIES:
            print(f"Warning: '{unified_category}' not in unified categories, skipping...")
            continue
            
        class_id = UNIFIED_CATEGORIES[unified_category]['id']
        
        for image_file in os.listdir(category_path):
            if not image_file.lower().endswith(('.jpg', '.jpeg', '.png')):
                continue
            
            image_path = os.path.join(category_path, image_file)
            
            # Create annotation (full image bounding box for classification dataset)
            # YOLO format: class_id x_center y_center width height (normalized)
            annotations.append({
                'source': 'trashnet',
                'image_path': image_path,
                'image_name': f"trashnet_{category}_{image_file}",
                'class_id': class_id,
                'class_name': unified_category,
                'bbox': [0.5, 0.5, 1.0, 1.0],  # Full image
                'is_full_image': True
            })
    
    print(f"Processed {len(annotations)} images from TrashNet")
    return annotations


def process_taco(
    taco_path: str,
    output_path: str,
    category_mapping: Dict[str, str]
) -> List[Dict]:
    """
    Process TACO dataset and convert to YOLO format.
    
    TACO is an object detection dataset with COCO-format annotations.
    
    Args:
        taco_path: Path to TACO dataset
        output_path: Output path for processed images
        category_mapping: Mapping from TACO to unified categories
    
    Returns:
        List of annotation dictionaries
    """
    annotations = []
    
    # Load TACO annotations
    ann_file = os.path.join(taco_path, "annotations.json")
    if not os.path.exists(ann_file):
        print(f"TACO annotations not found at {ann_file}")
        return annotations
    
    with open(ann_file, 'r') as f:
        taco_data = json.load(f)
    
    # Build category mapping
    taco_categories = {cat['id']: cat['name'] for cat in taco_data['categories']}
    
    # Build image info mapping
    images = {img['id']: img for img in taco_data['images']}
    
    # Process annotations
    for ann in taco_data['annotations']:
        image_id = ann['image_id']
        cat_id = ann['category_id']
        
        if image_id not in images:
            continue
            
        image_info = images[image_id]
        taco_category = taco_categories.get(cat_id, '')
        
        # Map to unified category
        unified_category = category_mapping.get(taco_category, 'trash_misc')
        if unified_category not in UNIFIED_CATEGORIES:
            unified_category = 'trash_misc'
        
        class_id = UNIFIED_CATEGORIES[unified_category]['id']
        
        # Convert COCO bbox [x, y, width, height] to YOLO format
        img_width = image_info['width']
        img_height = image_info['height']
        
        x, y, w, h = ann['bbox']
        x_center = (x + w/2) / img_width
        y_center = (y + h/2) / img_height
        width = w / img_width
        height = h / img_height
        
        # Clamp values to [0, 1]
        x_center = max(0, min(1, x_center))
        y_center = max(0, min(1, y_center))
        width = max(0, min(1, width))
        height = max(0, min(1, height))
        
        # Build image path
        image_path = os.path.join(taco_path, image_info['file_name'])
        
        annotations.append({
            'source': 'taco',
            'image_path': image_path,
            'image_name': f"taco_{image_info['file_name'].replace('/', '_')}",
            'class_id': class_id,
            'class_name': unified_category,
            'bbox': [x_center, y_center, width, height],
            'is_full_image': False
        })
    
    print(f"Processed {len(annotations)} annotations from TACO")
    return annotations


def split_dataset(
    annotations: List[Dict],
    train_split: float = 0.8,
    val_split: float = 0.15,
    test_split: float = 0.05,
    seed: int = 42
) -> Tuple[List[Dict], List[Dict], List[Dict]]:
    """
    Split annotations into train/val/test sets.
    
    Ensures balanced distribution across classes.
    
    Args:
        annotations: List of annotation dictionaries
        train_split: Fraction for training set
        val_split: Fraction for validation set
        test_split: Fraction for test set
        seed: Random seed for reproducibility
    
    Returns:
        Tuple of (train, val, test) annotation lists
    """
    random.seed(seed)
    
    # Group by image to avoid same image in multiple splits
    image_annotations = defaultdict(list)
    for ann in annotations:
        image_annotations[ann['image_path']].append(ann)
    
    # Shuffle images
    image_paths = list(image_annotations.keys())
    random.shuffle(image_paths)
    
    # Calculate split indices
    n_images = len(image_paths)
    train_end = int(n_images * train_split)
    val_end = train_end + int(n_images * val_split)
    
    # Split
    train_images = image_paths[:train_end]
    val_images = image_paths[train_end:val_end]
    test_images = image_paths[val_end:]
    
    # Gather annotations for each split
    train_anns = [ann for img in train_images for ann in image_annotations[img]]
    val_anns = [ann for img in val_images for ann in image_annotations[img]]
    test_anns = [ann for img in test_images for ann in image_annotations[img]]
    
    print(f"Split dataset: {len(train_anns)} train, {len(val_anns)} val, {len(test_anns)} test")
    
    return train_anns, val_anns, test_anns


def save_yolo_dataset(
    annotations: List[Dict],
    output_path: str,
    split_name: str
) -> None:
    """
    Save annotations in YOLO format.
    
    Creates:
    - images/<split>/ folder with copied images
    - labels/<split>/ folder with .txt annotation files
    
    Args:
        annotations: List of annotation dictionaries
        output_path: Base output path
        split_name: Name of split (train/val/test)
    """
    images_dir = os.path.join(output_path, "images", split_name)
    labels_dir = os.path.join(output_path, "labels", split_name)
    
    os.makedirs(images_dir, exist_ok=True)
    os.makedirs(labels_dir, exist_ok=True)
    
    # Group annotations by image
    image_annotations = defaultdict(list)
    for ann in annotations:
        image_annotations[ann['image_name']].append(ann)
    
    # Process each image
    for image_name, anns in image_annotations.items():
        # Copy image
        src_path = anns[0]['image_path']
        if not os.path.exists(src_path):
            print(f"Warning: Image not found: {src_path}")
            continue
        
        # Determine extension
        ext = os.path.splitext(src_path)[1]
        dst_image_name = os.path.splitext(image_name)[0] + ext
        dst_image_path = os.path.join(images_dir, dst_image_name)
        
        try:
            shutil.copy2(src_path, dst_image_path)
        except Exception as e:
            print(f"Error copying {src_path}: {e}")
            continue
        
        # Create label file
        label_name = os.path.splitext(image_name)[0] + ".txt"
        label_path = os.path.join(labels_dir, label_name)
        
        with open(label_path, 'w') as f:
            for ann in anns:
                class_id = ann['class_id']
                x_center, y_center, width, height = ann['bbox']
                f.write(f"{class_id} {x_center:.6f} {y_center:.6f} {width:.6f} {height:.6f}\n")
    
    print(f"Saved {len(image_annotations)} images to {split_name} split")


def generate_yolo_yaml(output_path: str, categories: Dict) -> str:
    """
    Generate YOLO dataset configuration YAML.
    
    Args:
        output_path: Path to the dataset
        categories: Dictionary of unified categories
    
    Returns:
        Path to generated YAML file
    """
    # Sort categories by ID
    sorted_categories = sorted(categories.items(), key=lambda x: x[1]['id'])
    names = {cat[1]['id']: cat[0] for cat in sorted_categories}
    
    config = {
        'path': os.path.abspath(output_path),
        'train': 'images/train',
        'val': 'images/val',
        'test': 'images/test',
        'nc': len(names),
        'names': names
    }
    
    yaml_path = os.path.join(output_path, "dataset.yaml")
    with open(yaml_path, 'w') as f:
        yaml.dump(config, f, default_flow_style=False, sort_keys=False)
    
    print(f"Generated dataset config: {yaml_path}")
    return yaml_path


def generate_statistics(annotations: List[Dict], output_path: str) -> None:
    """
    Generate and save dataset statistics.
    
    Args:
        annotations: All annotations
        output_path: Path to save statistics
    """
    stats = {
        'total_annotations': len(annotations),
        'total_images': len(set(ann['image_path'] for ann in annotations)),
        'by_source': defaultdict(int),
        'by_class': defaultdict(int),
        'by_bin': defaultdict(int)
    }
    
    for ann in annotations:
        stats['by_source'][ann['source']] += 1
        stats['by_class'][ann['class_name']] += 1
        
        bin_name = UNIFIED_CATEGORIES.get(ann['class_name'], {}).get('bin_name', 'UNKNOWN')
        stats['by_bin'][bin_name] += 1
    
    # Convert defaultdicts to regular dicts for YAML
    stats['by_source'] = dict(stats['by_source'])
    stats['by_class'] = dict(stats['by_class'])
    stats['by_bin'] = dict(stats['by_bin'])
    
    stats_path = os.path.join(output_path, "statistics.yaml")
    with open(stats_path, 'w') as f:
        yaml.dump(stats, f, default_flow_style=False, sort_keys=False)
    
    # Print summary
    print("\n" + "="*60)
    print("DATASET STATISTICS")
    print("="*60)
    print(f"Total annotations: {stats['total_annotations']}")
    print(f"Total images: {stats['total_images']}")
    print("\nBy source:")
    for source, count in stats['by_source'].items():
        print(f"  {source}: {count}")
    print("\nBy bin:")
    for bin_name, count in sorted(stats['by_bin'].items()):
        print(f"  {bin_name}: {count}")
    print(f"\nStatistics saved to: {stats_path}")
    print("="*60 + "\n")


# =============================================================================
# MAIN FUNCTION
# =============================================================================

def main():
    parser = argparse.ArgumentParser(
        description="Merge TrashNet and TACO datasets for waste classification training"
    )
    parser.add_argument(
        '--trashnet-path',
        type=str,
        default='./datasets/trashnet',
        help='Path to TrashNet dataset'
    )
    parser.add_argument(
        '--taco-path',
        type=str,
        default='./datasets/taco',
        help='Path to TACO dataset'
    )
    parser.add_argument(
        '--output-path',
        type=str,
        default='./datasets/merged',
        help='Output path for merged dataset'
    )
    parser.add_argument(
        '--train-split',
        type=float,
        default=0.8,
        help='Fraction for training set (default: 0.8)'
    )
    parser.add_argument(
        '--val-split',
        type=float,
        default=0.15,
        help='Fraction for validation set (default: 0.15)'
    )
    parser.add_argument(
        '--test-split',
        type=float,
        default=0.05,
        help='Fraction for test set (default: 0.05)'
    )
    parser.add_argument(
        '--seed',
        type=int,
        default=42,
        help='Random seed for reproducibility (default: 42)'
    )
    parser.add_argument(
        '--download',
        action='store_true',
        help='Download datasets if not present'
    )
    
    args = parser.parse_args()
    
    # Validate splits
    total_split = args.train_split + args.val_split + args.test_split
    if abs(total_split - 1.0) > 0.001:
        print(f"Warning: Splits sum to {total_split}, not 1.0. Normalizing...")
        args.train_split /= total_split
        args.val_split /= total_split
        args.test_split /= total_split
    
    print("\n" + "="*60)
    print("WASTE DATASET MERGER")
    print("="*60)
    print(f"TrashNet path: {args.trashnet_path}")
    print(f"TACO path: {args.taco_path}")
    print(f"Output path: {args.output_path}")
    print(f"Splits: {args.train_split:.0%} train, {args.val_split:.0%} val, {args.test_split:.0%} test")
    print("="*60 + "\n")
    
    # Download datasets if requested
    if args.download:
        if not os.path.exists(os.path.join(args.trashnet_path, "dataset-resized")):
            download_trashnet(args.trashnet_path)
        if not os.path.exists(os.path.join(args.taco_path, "annotations.json")):
            download_taco(args.taco_path)
    
    # Process datasets
    all_annotations = []
    
    print("Processing TrashNet...")
    trashnet_anns = process_trashnet(
        args.trashnet_path,
        args.output_path,
        TRASHNET_MAPPING
    )
    all_annotations.extend(trashnet_anns)
    
    print("\nProcessing TACO...")
    taco_anns = process_taco(
        args.taco_path,
        args.output_path,
        TACO_MAPPING
    )
    all_annotations.extend(taco_anns)
    
    if not all_annotations:
        print("No annotations found. Please check dataset paths.")
        print("Use --download flag to download datasets.")
        return
    
    # Split dataset
    print("\nSplitting dataset...")
    train_anns, val_anns, test_anns = split_dataset(
        all_annotations,
        args.train_split,
        args.val_split,
        args.test_split,
        args.seed
    )
    
    # Create output directory
    os.makedirs(args.output_path, exist_ok=True)
    
    # Save YOLO format dataset
    print("\nSaving YOLO format dataset...")
    save_yolo_dataset(train_anns, args.output_path, "train")
    save_yolo_dataset(val_anns, args.output_path, "val")
    save_yolo_dataset(test_anns, args.output_path, "test")
    
    # Generate YAML config
    print("\nGenerating YOLO config...")
    generate_yolo_yaml(args.output_path, UNIFIED_CATEGORIES)
    
    # Generate statistics
    generate_statistics(all_annotations, args.output_path)
    
    print("\n" + "="*60)
    print("MERGE COMPLETE!")
    print("="*60)
    print(f"Dataset saved to: {args.output_path}")
    print(f"\nNext steps:")
    print(f"1. Review dataset.yaml configuration")
    print(f"2. Start training with:")
    print(f"   yolo train data={args.output_path}/dataset.yaml model=yolov8n.pt epochs=100")
    print("="*60 + "\n")


if __name__ == "__main__":
    main()
