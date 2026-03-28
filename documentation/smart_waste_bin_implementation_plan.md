# Smart Waste Bin Implementation Plan

## AI-Powered Waste Classification System using Jetson Orin Nano

**Version:** 1.0  
**Date:** January 2026  
**Based on:** Ameru.AI / ST6 Architecture  

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [System Architecture](#2-system-architecture)
3. [Hardware Requirements](#3-hardware-requirements)
   - [3.1 Edge Device](#31-edge-device)
   - [3.2 Jetson Orin Nano Complete Setup Guide](#32-jetson-orin-nano-complete-setup-guide)
4. [Software Stack](#4-software-stack)
5. [Waste Classification Taxonomy](#5-waste-classification-taxonomy)
6. [Phase 1: Data Acquisition](#6-phase-1-data-acquisition)
7. [Phase 2: Data Labeling](#7-phase-2-data-labeling)
8. [Phase 3: Model Training](#8-phase-3-model-training)
9. [Phase 4: Model Deployment](#9-phase-4-model-deployment)
10. [Phase 5: Active Learning Loop](#10-phase-5-active-learning-loop)
11. [Project Timeline](#11-project-timeline)
12. [File Structure](#12-file-structure)
13. [Appendix](#13-appendix)

---

## 1. Executive Summary

This document outlines a complete implementation plan for building an AI-powered smart waste bin that automatically classifies and sorts waste into six categories:

| Bin | Color | Contents |
|-----|-------|----------|
| **Plastic** | Blue | Recyclable plastics (PET, HDPE, PP) |
| **Metal** | Silver | Aluminum cans, steel cans, foil |
| **Glass** | Green | Glass bottles and jars |
| **Paper** | Yellow | Paper, cardboard, magazines |
| **Compost** | Brown | Food waste, organics |
| **Trash** | Black | Non-recyclable waste |

The system uses computer vision and deep learning to identify waste items in real-time, achieving 90%+ sorting accuracy through continuous active learning.

### Key Performance Targets

- **Inference Speed:** < 100ms per detection
- **Sorting Accuracy:** > 90% (targeting 95%)
- **Categories:** 60+ object types mapped to 6 bins
- **Model Size:** Optimized for edge deployment (~10MB)

---

## 2. System Architecture

### 2.1 High-Level Data Engine

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           SMART WASTE BIN DATA ENGINE                        │
└─────────────────────────────────────────────────────────────────────────────┘

┌──────────────┐    ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│     1.       │    │     2.       │    │     3.       │    │     4.       │
│    DATA      │───▶│    DATA      │───▶│    MODEL     │───▶│    MODEL     │
│ ACQUISITION  │    │   LABELING   │    │   TRAINING   │    │  DEPLOYMENT  │
│              │    │              │    │              │    │              │
│ Jetson Orin  │    │ Label Studio │    │ PyTorch/YOLO │    │  TensorRT    │
│ + Camera     │    │ + S3 Storage │    │ + Lambda Labs│    │  Optimized   │
└──────────────┘    └──────────────┘    └──────────────┘    └──────────────┘
       ▲                                                            │
       │                    ┌──────────────┐                        │
       │                    │     5.       │                        │
       └────────────────────│   ACTIVE     │◀───────────────────────┘
                            │  LEARNING    │
                            │              │
                            │ Continuous   │
                            │ Improvement  │
                            └──────────────┘
```

### 2.2 Component Interaction Diagram

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         JETSON ORIN NANO                                │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                 │
│  │   Camera    │───▶│   Image     │───▶│  TensorRT   │                 │
│  │   (8MP)     │    │  Capture    │    │  Inference  │                 │
│  └─────────────┘    └─────────────┘    └──────┬──────┘                 │
│                                               │                         │
│                     ┌─────────────────────────┴────────────────┐       │
│                     │                                          │       │
│                     ▼                                          ▼       │
│            ┌─────────────┐                           ┌─────────────┐   │
│            │   Local     │                           │   Sorting   │   │
│            │   Storage   │                           │   Logic     │   │
│            │  /images/   │                           │   + Servo   │   │
│            └──────┬──────┘                           └─────────────┘   │
└───────────────────┼─────────────────────────────────────────────────────┘
                    │
                    │ Periodic Sync (cron)
                    ▼
          ┌─────────────────────┐
          │     AWS S3          │
          │  waste-images/      │
          │  ├── raw/           │
          │  ├── labeled/       │
          │  └── review_queue/  │
          └──────────┬──────────┘
                     │
        ┌────────────┴────────────┐
        ▼                         ▼
┌───────────────┐        ┌───────────────┐
│  Label Studio │        │ Lambda Labs   │
│  (Labeling)   │        │ (Training)    │
└───────────────┘        └───────────────┘
```

---

## 3. Hardware Requirements

### 3.1 Edge Device

| Component | Specification | Notes |
|-----------|--------------|-------|
| **Compute** | NVIDIA Jetson Orin Nano | 8GB RAM, 40 TOPS AI performance |
| **Camera** | IMX219 (8MP) or IMX477 (12MP) | CSI-2 interface, 1080p @ 30fps |
| **Storage** | 512GB microSD card (or NVMe SSD) | For OS, models, and image caching |
| **Power** | 15W adapter | Low power consumption |
| **Enclosure** | Custom 3D printed | Weather-resistant if outdoor |

---

## 3.2 Jetson Orin Nano Complete Setup Guide

This section provides step-by-step instructions for setting up a new NVIDIA Jetson Orin Nano Developer Kit with a 512GB microSD card.

### 3.2.1 What You'll Need

| Item | Description | Notes |
|------|-------------|-------|
| Jetson Orin Nano Developer Kit | 8GB version recommended | Includes carrier board |
| 512GB microSD Card | High-speed UHS-I or UHS-II | Samsung EVO Plus or SanDisk Extreme recommended |
| Power Supply | USB-C 45W+ or DC barrel jack | Must support 15W+ delivery |
| Display | HDMI monitor | For initial setup |
| Keyboard & Mouse | USB | For initial configuration |
| Ethernet Cable | Cat5e or better | For initial downloads (faster than WiFi) |
| Host Computer | Ubuntu 20.04/22.04, Windows, or macOS | For flashing SD card |
| microSD Card Reader | USB 3.0 recommended | For flashing |
| CSI Camera | IMX219 or IMX477 | Optional: for immediate testing |

### 3.2.2 Step 1: Download JetPack SDK

JetPack is NVIDIA's comprehensive SDK that includes the OS, CUDA, cuDNN, TensorRT, and other libraries.

**Option A: Download SD Card Image (Recommended for beginners)**

1. Visit the NVIDIA JetPack download page:
   ```
   https://developer.nvidia.com/embedded/jetpack
   ```

2. Download the latest JetPack for Jetson Orin Nano:
   - Look for "Jetson Orin Nano Developer Kit"
   - Download the SD Card Image (approximately 15-18GB)
   - Current recommended version: JetPack 6.0 or later

3. Verify the download (optional but recommended):
   ```bash
   # On Linux/macOS
   sha256sum jetson-orin-nano-devkit-sd-card-image.zip
   
   # Compare with checksum on NVIDIA website
   ```

**Option B: Use SDK Manager (More control)**

1. Download NVIDIA SDK Manager:
   ```
   https://developer.nvidia.com/sdk-manager
   ```

2. Install SDK Manager:
   ```bash
   # Ubuntu
   sudo apt install ./sdkmanager_*_amd64.deb
   ```

3. Run SDK Manager and follow the GUI to flash the Jetson

### 3.2.3 Step 2: Flash the 512GB microSD Card

**On Linux:**

```bash
# 1. Identify your SD card device
lsblk
# Look for your 512GB card (e.g., /dev/sdb or /dev/mmcblk0)
# BE VERY CAREFUL to identify the correct device!

# 2. Unzip the image
unzip jetson-orin-nano-devkit-sd-card-image.zip

# 3. Flash the SD card (replace /dev/sdX with your device)
sudo dd if=sd-blob.img of=/dev/sdX bs=4M status=progress
sudo sync

# 4. Eject safely
sudo eject /dev/sdX
```

**On Windows:**

1. Download and install **balenaEtcher**:
   ```
   https://www.balena.io/etcher/
   ```

2. Run Etcher:
   - Click "Flash from file" → Select the downloaded image
   - Click "Select target" → Choose your 512GB SD card
   - Click "Flash!" and wait for completion

**On macOS:**

```bash
# 1. Identify your SD card
diskutil list
# Look for your 512GB card (e.g., /dev/disk4)

# 2. Unmount the disk
diskutil unmountDisk /dev/diskN

# 3. Flash the image
sudo dd if=sd-blob.img of=/dev/rdiskN bs=4m
# Note: Use 'rdisk' (raw disk) for faster writes

# 4. Eject
diskutil eject /dev/diskN
```

### 3.2.4 Step 3: First Boot and Initial Configuration

1. **Insert the SD card** into the Jetson Orin Nano (slot is on the bottom of the module)

2. **Connect peripherals:**
   - HDMI cable to monitor
   - USB keyboard and mouse
   - Ethernet cable (recommended)
   - Power supply (connect last)

3. **Power on** the Jetson:
   - The green LED should light up
   - Wait for the NVIDIA logo and Ubuntu desktop to appear (2-3 minutes)

4. **Complete the setup wizard:**
   ```
   - Accept license agreement
   - Select language: English (or your preference)
   - Select keyboard layout
   - Select timezone
   - Create user account:
       Username: jetson (or your preference)
       Password: [choose a strong password]
       Computer name: waste-bin-001 (or your preference)
   - Configure network (if not using Ethernet)
   - Wait for initial setup to complete
   ```

5. **Reboot** when prompted

### 3.2.5 Step 4: Expand Filesystem to Use Full 512GB

By default, the flashed image only uses a portion of the SD card. Expand to use the full 512GB:

```bash
# Check current disk usage
df -h

# You'll see something like:
# /dev/mmcblk0p1   58G   15G   40G  28% /

# Expand the partition to use full SD card
sudo apt update
sudo apt install -y gparted

# Option 1: Use gparted GUI
sudo gparted
# Select /dev/mmcblk0p1 → Resize/Move → Drag to fill all space → Apply

# Option 2: Use command line
sudo parted /dev/mmcblk0 resizepart 1 100%
sudo resize2fs /dev/mmcblk0p1

# Verify the new size
df -h
# Should now show ~475GB available
```

### 3.2.6 Step 5: System Update and Essential Packages

```bash
# Update package lists
sudo apt update

# Upgrade all packages
sudo apt upgrade -y

# Install essential development tools
sudo apt install -y \
    build-essential \
    cmake \
    git \
    curl \
    wget \
    vim \
    nano \
    htop \
    iotop \
    net-tools \
    openssh-server \
    python3-pip \
    python3-venv \
    python3-dev \
    libopenblas-dev \
    libopencv-dev \
    v4l-utils

# Reboot to apply any kernel updates
sudo reboot
```

### 3.2.7 Step 6: Verify CUDA and TensorRT Installation

JetPack comes with CUDA, cuDNN, and TensorRT pre-installed. Verify:

```bash
# Check CUDA version
nvcc --version
# Expected: CUDA 12.2 or later

# Check TensorRT version
dpkg -l | grep tensorrt
# Expected: TensorRT 8.6 or later

# Check cuDNN version
cat /usr/include/cudnn_version.h | grep CUDNN_MAJOR -A 2

# Test CUDA with a sample
cd /usr/local/cuda/samples/1_Utilities/deviceQuery
sudo make
./deviceQuery
# Should show "Result = PASS"

# Check Jetson stats
sudo tegrastats
# Shows CPU, GPU, memory usage in real-time (Ctrl+C to exit)
```

### 3.2.8 Step 7: Configure Power Mode

The Jetson Orin Nano has multiple power modes. For best performance:

```bash
# Check current power mode
sudo nvpmodel -q

# Set to maximum performance (15W mode)
sudo nvpmodel -m 0

# Enable maximum CPU/GPU clocks
sudo jetson_clocks

# To make jetson_clocks persistent across reboots:
sudo jetson_clocks --store
# Then add to /etc/rc.local or create a systemd service
```

### 3.2.9 Step 8: Set Up Python Environment for AI/ML

```bash
# Upgrade pip
pip3 install --upgrade pip

# Install PyTorch for Jetson (special wheel from NVIDIA)
# Check https://forums.developer.nvidia.com/t/pytorch-for-jetson/ for latest
# Example for JetPack 6.0:
pip3 install --no-cache \
    https://developer.download.nvidia.com/compute/redist/jp/v60/pytorch/torch-2.1.0a0+41361538.nv23.06-cp310-cp310-linux_aarch64.whl

# Install torchvision (must compile from source for ARM)
sudo apt install -y libjpeg-dev zlib1g-dev libpython3-dev
git clone --branch v0.16.0 https://github.com/pytorch/vision torchvision
cd torchvision
python3 setup.py install --user
cd ..

# Install Ultralytics YOLOv8
pip3 install ultralytics

# Install other ML dependencies
pip3 install \
    numpy \
    opencv-python \
    pillow \
    scipy \
    matplotlib \
    pandas \
    pyyaml \
    tqdm \
    boto3 \
    requests

# Verify PyTorch CUDA support
python3 -c "import torch; print(f'PyTorch: {torch.__version__}'); print(f'CUDA available: {torch.cuda.is_available()}')"
```

### 3.2.10 Step 9: Configure the CSI Camera

```bash
# Check if camera is detected
ls /dev/video*
# Should show /dev/video0

# Test camera with GStreamer (IMX219)
gst-launch-1.0 nvarguscamerasrc sensor-id=0 ! \
    'video/x-raw(memory:NVMM), width=1920, height=1080, framerate=30/1' ! \
    nvvidconv ! xvimagesink

# For IMX477 (higher resolution)
gst-launch-1.0 nvarguscamerasrc sensor-id=0 ! \
    'video/x-raw(memory:NVMM), width=4032, height=3040, framerate=30/1' ! \
    nvvidconv ! xvimagesink

# Test with OpenCV Python
python3 << 'EOF'
import cv2

# GStreamer pipeline for CSI camera
gst_pipeline = (
    "nvarguscamerasrc sensor-id=0 ! "
    "video/x-raw(memory:NVMM), width=1920, height=1080, framerate=30/1 ! "
    "nvvidconv ! video/x-raw, format=BGRx ! "
    "videoconvert ! video/x-raw, format=BGR ! "
    "appsink drop=1"
)

cap = cv2.VideoCapture(gst_pipeline, cv2.CAP_GSTREAMER)

if cap.isOpened():
    ret, frame = cap.read()
    if ret:
        print(f"Camera working! Frame shape: {frame.shape}")
        cv2.imwrite("test_capture.jpg", frame)
        print("Saved test_capture.jpg")
    cap.release()
else:
    print("Failed to open camera")
EOF

# View the captured image
display test_capture.jpg  # Or use any image viewer
```

### 3.2.11 Step 10: Configure Remote Access (SSH)

```bash
# SSH should be installed, but verify
sudo systemctl status ssh

# If not running, start and enable
sudo systemctl start ssh
sudo systemctl enable ssh

# Get your IP address
ip addr show eth0  # For Ethernet
ip addr show wlan0  # For WiFi

# From another computer, connect:
# ssh jetson@<IP_ADDRESS>

# Set up SSH keys for passwordless login (on your host computer)
ssh-keygen -t ed25519 -C "your_email@example.com"
ssh-copy-id jetson@<JETSON_IP>
```

### 3.2.12 Step 11: Configure Storage Structure

Set up the directory structure for the waste bin project:

```bash
# Create project directories
mkdir -p ~/waste_bin/{capture,sync,inference,logs,models}
mkdir -p ~/images/{raw,review_queue,processed}
mkdir -p ~/datasets/{trashnet,taco,merged}

# Set permissions
chmod -R 755 ~/waste_bin
chmod -R 755 ~/images

# Check available storage
df -h /home/jetson
# Should show ~450GB+ available on your 512GB card

# Create a RAM disk for temporary processing (optional, for speed)
sudo mkdir -p /mnt/ramdisk
sudo mount -t tmpfs -o size=2G tmpfs /mnt/ramdisk

# To make RAM disk permanent, add to /etc/fstab:
echo "tmpfs /mnt/ramdisk tmpfs size=2G 0 0" | sudo tee -a /etc/fstab
```

### 3.2.13 Step 12: Set Up Automatic Startup (Optional)

Create a systemd service to start the waste detection on boot:

```bash
# Create the service file
sudo tee /etc/systemd/system/waste-detector.service << 'EOF'
[Unit]
Description=Waste Detection Service
After=network.target

[Service]
Type=simple
User=jetson
WorkingDirectory=/home/jetson/waste_bin
ExecStart=/usr/bin/python3 /home/jetson/waste_bin/inference/detector.py
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

# Reload systemd and enable the service
sudo systemctl daemon-reload
sudo systemctl enable waste-detector.service

# Start the service (after you've created the detector script)
# sudo systemctl start waste-detector.service

# Check status
# sudo systemctl status waste-detector.service
```

### 3.2.14 Step 13: Performance Optimization

```bash
# Disable GUI for headless operation (saves ~500MB RAM)
sudo systemctl set-default multi-user.target
# To re-enable: sudo systemctl set-default graphical.target

# Disable unnecessary services
sudo systemctl disable apt-daily.service
sudo systemctl disable apt-daily-upgrade.service
sudo systemctl disable ModemManager.service

# Set CPU governor to performance
echo 'GOVERNOR="performance"' | sudo tee /etc/default/cpufrequtils
sudo systemctl restart cpufrequtils

# Increase swap size for large model training (optional)
sudo fallocate -l 8G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab

# Verify swap
free -h
```

### 3.2.15 Step 14: Test YOLOv8 Inference

```bash
# Quick test with a pretrained model
python3 << 'EOF'
from ultralytics import YOLO

# Download and load YOLOv8 nano model
model = YOLO('yolov8n.pt')

# Run inference on a test image (or your camera capture)
results = model.predict(source='test_capture.jpg', save=True, conf=0.5)

# Print results
for r in results:
    print(f"Detected {len(r.boxes)} objects")
    for box in r.boxes:
        cls = int(box.cls[0])
        conf = float(box.conf[0])
        print(f"  - {model.names[cls]}: {conf:.2f}")

# Check inference speed
model.benchmark(imgsz=640, half=True, device=0)
EOF

# Results will be saved in runs/detect/predict/
```

### 3.2.16 Jetson Orin Nano Setup Checklist

Use this checklist to verify your setup is complete:

- [ ] SD card flashed with JetPack 6.0+
- [ ] First boot completed, user account created
- [ ] Filesystem expanded to use full 512GB
- [ ] System updated (`apt update && apt upgrade`)
- [ ] CUDA verified (`nvcc --version`)
- [ ] TensorRT verified (`dpkg -l | grep tensorrt`)
- [ ] Power mode set to 15W (`nvpmodel -m 0`)
- [ ] Python environment configured
- [ ] PyTorch installed with CUDA support
- [ ] Ultralytics YOLOv8 installed
- [ ] CSI camera tested and working
- [ ] SSH access configured
- [ ] Project directories created
- [ ] YOLOv8 inference test passed

### 3.2.17 Troubleshooting Common Issues

| Issue | Solution |
|-------|----------|
| **Jetson won't boot** | Re-flash SD card, try different card, check power supply |
| **"No CUDA device"** | Run `sudo nvpmodel -m 0` then reboot |
| **Camera not detected** | Check ribbon cable connection, ensure camera facing correct way |
| **Out of memory** | Close GUI (`sudo systemctl stop gdm3`), increase swap |
| **Slow inference** | Ensure TensorRT model is used, not PyTorch directly |
| **SD card corrupted** | Use high-quality card, ensure proper shutdown |
| **SSH connection refused** | Run `sudo systemctl start ssh` |
| **PyTorch CUDA error** | Install Jetson-specific PyTorch wheel from NVIDIA |

### 3.2.18 Useful Monitoring Commands

```bash
# Real-time system monitoring
sudo tegrastats  # Jetson-specific stats

# GPU monitoring
watch -n 1 nvidia-smi  # Or use jtop

# Install jtop (better than tegrastats)
sudo pip3 install jetson-stats
sudo systemctl restart jtop.service
jtop  # Beautiful terminal UI for monitoring

# Disk I/O monitoring
sudo iotop

# Network monitoring
sudo iftop

# Temperature monitoring
cat /sys/devices/virtual/thermal/thermal_zone*/temp
```

---

### 3.2 Bin Mechanism (Optional)

| Component | Specification |
|-----------|--------------|
| Servo Motors | 2x MG996R or similar |
| Sorting Tray | Pan-tilt mechanism |
| Compartments | 4-6 separate bins (40L each) |
| Display | 10.1" touchscreen (optional) |

### 3.3 Cloud Infrastructure

| Service | Purpose | Estimated Cost |
|---------|---------|----------------|
| AWS S3 | Image storage | ~$5/month |
| Lambda Labs | GPU training | ~$1.10/hour (A10) |
| Label Studio | Self-hosted or cloud | Free (self-hosted) |

---

## 4. Software Stack

### 4.1 Development Environment

```bash
# Jetson Orin Nano
JetPack SDK 6.0+
CUDA 12.2
cuDNN 8.9
TensorRT 8.6
Python 3.10

# Training Server
PyTorch 2.0+
Ultralytics YOLOv8
ONNX 1.14+
Weights & Biases
```

### 4.2 Key Libraries

| Library | Version | Purpose |
|---------|---------|---------|
| `ultralytics` | 8.0+ | YOLO training & export |
| `opencv-python` | 4.8+ | Image processing |
| `label-studio` | 1.10+ | Data labeling |
| `boto3` | 1.28+ | AWS S3 sync |
| `tensorrt` | 8.6+ | Inference optimization |
| `wandb` | 0.15+ | Experiment tracking |

### 4.3 Installation Commands

```bash
# On Jetson Orin Nano
sudo apt update && sudo apt upgrade -y
sudo apt install python3-pip python3-opencv -y

pip install ultralytics boto3 pyyaml

# On Training Server
pip install torch torchvision --index-url https://download.pytorch.org/whl/cu118
pip install ultralytics wandb label-studio
```

---

## 5. Waste Classification Taxonomy

### 5.1 Multi-Dimensional Classification Schema

The system uses a **Material × Object × Condition** approach, creating granular categories that map to final bins.

```
DETECTION OUTPUT
      │
      ├── Material: PET
      ├── Object: water_bottle
      └── Condition: clean
              │
              ▼
      ┌─────────────────┐
      │ Decision Logic  │
      │                 │
      │ IF clean        │
      │   → PLASTIC     │
      │ IF soiled       │
      │   → TRASH       │
      └─────────────────┘
```

### 5.2 Materials (17 types)

| Material | Recyclable | Default Bin |
|----------|------------|-------------|
| PET | ✅ | Plastic |
| HDPE | ✅ | Plastic |
| PVC | ❌ | Trash |
| LDPE | ❌* | Trash |
| PP | ✅ | Plastic |
| PS (Styrofoam) | ❌ | Trash |
| OTHER_PLASTIC | ❌ | Trash |
| ALUMINUM | ✅ | Metal |
| STEEL | ✅ | Metal |
| GLASS | ✅ | Glass |
| PAPER | ✅ | Paper |
| CARDBOARD | ✅ | Paper |
| ORGANIC | ✅ | Compost |
| COMPOSITE | ❌ | Trash |
| TEXTILE | ❌ | Trash |

*LDPE bags require special collection, not curbside recycling

### 5.3 Objects (60+ types)

See `config/waste_taxonomy.yaml` for the complete list organized by category.

### 5.4 Conditions (5 states)

| Condition | Effect on Sorting |
|-----------|------------------|
| `clean` | Use default bin |
| `soiled` | Recyclables → Trash (except paper → Compost) |
| `wet` | Paper/Cardboard → Trash |
| `crushed` | No change |
| `damaged` | No change |

### 5.5 Final Category Count

| Bin | Categories |
|-----|------------|
| Plastic | 15 |
| Metal | 10 |
| Glass | 8 |
| Paper | 15 |
| Compost | 18 |
| Trash | 17 |
| **Total** | **83** |

---

## 6. Phase 1: Data Acquisition

### 6.1 Objectives

- Set up camera and capture pipeline on Jetson Orin Nano
- Implement automatic cloud sync to AWS S3
- Create logging system for all captures

### 6.2 Directory Structure on Jetson

```
/home/jetson/
├── waste_bin/
│   ├── capture/
│   │   ├── capture.py
│   │   └── config.yaml
│   ├── sync/
│   │   ├── s3_sync.py
│   │   └── credentials.yaml
│   ├── inference/
│   │   ├── detector.py
│   │   └── model.engine
│   └── logs/
│       ├── captures.log
│       └── predictions.log
├── images/
│   ├── raw/           # All captured images
│   └── review_queue/  # Low-confidence predictions
└── models/
    └── waste_detector.engine
```

### 6.3 Camera Setup

```python
# capture/config.yaml
camera:
  device: 0  # CSI camera
  resolution: [1920, 1080]
  fps: 30
  format: "MJPG"

capture:
  trigger: "motion"  # or "continuous" or "button"
  interval_ms: 500
  save_path: "/home/jetson/images/raw"
  
s3:
  bucket: "waste-bin-images"
  prefix: "device-001"
  sync_interval_minutes: 60
```

### 6.4 Implementation Files

See `scripts/capture.py` for the complete capture implementation.

### 6.5 Cron Job for S3 Sync

```bash
# Add to crontab -e
0 * * * * /usr/bin/python3 /home/jetson/waste_bin/sync/s3_sync.py >> /home/jetson/waste_bin/logs/sync.log 2>&1
```

---

## 7. Phase 2: Data Labeling

### 7.1 Objectives

- Deploy Label Studio for annotation
- Create labeling configuration with Material × Object × Condition schema
- Set up ML-assisted labeling backend

### 7.2 Label Studio Setup

```bash
# Install Label Studio
pip install label-studio

# Start Label Studio
label-studio start --host 0.0.0.0 --port 8080

# Or use Docker
docker run -it -p 8080:8080 -v $(pwd)/mydata:/label-studio/data heartexlabs/label-studio:latest
```

### 7.3 Labeling Configuration

See `config/label_studio_config.xml` for the complete configuration.

The configuration supports:
- Bounding box annotation (for object detection)
- Multi-label classification (Material, Object, Condition)
- Hierarchical taxonomy

### 7.4 Connecting S3 Storage

1. In Label Studio, go to **Project Settings → Cloud Storage**
2. Add **Source Storage** (S3):
   - Bucket: `waste-bin-images`
   - Prefix: `raw/`
   - Region: `us-east-1`
   - Use IAM role or access keys

3. Add **Target Storage** (S3):
   - Bucket: `waste-bin-images`
   - Prefix: `labeled/`

### 7.5 Labeling Guidelines

| Rule | Description |
|------|-------------|
| Draw tight boxes | Bounding box should fit the object closely |
| Label all visible items | Even partial objects at edges |
| Mark condition accurately | Soiled = visible food residue |
| Use "unknown" sparingly | Only for truly unidentifiable items |

### 7.6 Bootstrap Strategy (Zero-Shot)

For initial model without labeled data, use GLIP or GroundingDINO:

```bash
# Clone GroundingDINO
git clone https://github.com/IDEA-Research/GroundingDINO.git
cd GroundingDINO
pip install -e .

# Use for zero-shot detection
python demo.py --image_path image.jpg --text_prompt "plastic bottle, aluminum can, paper cup"
```

---

## 8. Phase 3: Model Training

### 8.1 Objectives

- Merge and prepare training datasets
- Train YOLOv8 object detection model
- Export to ONNX format
- Track experiments with Weights & Biases

### 8.2 Dataset Preparation

#### 8.2.1 Bootstrap Datasets

| Dataset | Classes | Images | Download |
|---------|---------|--------|----------|
| TrashNet | 6 | 2,527 | github.com/garythung/trashnet |
| TACO | 60 | 1,500+ | tacodataset.org |

#### 8.2.2 Dataset Merger Script

See `scripts/merge_datasets.py` for the complete implementation.

```bash
# Run the merger
python scripts/merge_datasets.py \
    --trashnet-path ./datasets/trashnet \
    --taco-path ./datasets/taco \
    --output-path ./datasets/merged \
    --train-split 0.8 \
    --val-split 0.15 \
    --test-split 0.05
```

### 8.3 Training Configuration

See `config/yolo_training_config.yaml` for the complete configuration.

### 8.4 Training Script

```python
from ultralytics import YOLO
import wandb

# Initialize W&B
wandb.init(project="waste-detection", name="yolov8n-baseline")

# Load model
model = YOLO('yolov8n.pt')

# Train
results = model.train(
    data='config/yolo_training_config.yaml',
    epochs=100,
    imgsz=640,
    batch=16,
    device=0,
    project='runs/train',
    name='waste-detector-v1',
    patience=20,
    save=True,
    plots=True
)

# Export to ONNX
model.export(format='onnx', imgsz=640, simplify=True, opset=12)
```

### 8.5 Training on Lambda Labs

```bash
# SSH into Lambda Labs instance
ssh ubuntu@<instance-ip>

# Clone your repo
git clone https://github.com/yourusername/waste-detector.git
cd waste-detector

# Install dependencies
pip install -r requirements.txt

# Start training
python train.py
```

### 8.6 Expected Training Time

| Model | GPU | Epochs | Time |
|-------|-----|--------|------|
| YOLOv8n | A10 | 100 | ~2 hours |
| YOLOv8s | A10 | 100 | ~4 hours |
| YOLOv8m | A10 | 100 | ~8 hours |

---

## 9. Phase 4: Model Deployment

### 9.1 Objectives

- Convert ONNX model to TensorRT engine
- Deploy inference pipeline on Jetson
- Implement sorting logic

### 9.2 Model Conversion Pipeline

```
PyTorch (.pt)
      │
      ▼ model.export(format='onnx')
ONNX (.onnx)
      │
      ▼ trtexec --onnx=model.onnx
TensorRT (.engine)
      │
      ▼ Deploy to Jetson
Inference Ready
```

### 9.3 TensorRT Conversion on Jetson

```bash
# Convert ONNX to TensorRT
/usr/src/tensorrt/bin/trtexec \
    --onnx=models/waste_detector.onnx \
    --saveEngine=models/waste_detector.engine \
    --fp16 \
    --workspace=4096 \
    --verbose

# Or use Ultralytics directly
from ultralytics import YOLO
model = YOLO('models/waste_detector.pt')
model.export(format='engine', device=0, half=True)
```

### 9.4 Inference Pipeline

```python
# inference/detector.py
from ultralytics import YOLO
import cv2

class WasteDetector:
    def __init__(self, model_path):
        self.model = YOLO(model_path)
        self.bin_mapping = {
            0: 'PLASTIC', 1: 'METAL', 2: 'GLASS',
            3: 'PAPER', 4: 'COMPOST', 5: 'TRASH'
        }
    
    def detect(self, image):
        results = self.model.predict(image, conf=0.5)
        detections = []
        for r in results:
            for box in r.boxes:
                detections.append({
                    'class_id': int(box.cls[0]),
                    'class_name': self.model.names[int(box.cls[0])],
                    'confidence': float(box.conf[0]),
                    'bbox': box.xyxy[0].tolist()
                })
        return detections
    
    def get_bin(self, detection):
        # Apply sorting logic based on class and condition
        class_name = detection['class_name']
        return self.class_to_bin_mapping.get(class_name, 'TRASH')
```

### 9.5 Performance Benchmarks

| Model | Jetson Orin Nano | Inference Time |
|-------|------------------|----------------|
| YOLOv8n (FP16) | TensorRT | ~15ms |
| YOLOv8s (FP16) | TensorRT | ~25ms |
| YOLOv8m (FP16) | TensorRT | ~45ms |

---

## 10. Phase 5: Active Learning Loop

### 10.1 Objectives

- Implement prediction logging with confidence scores
- Create review queue for low-confidence predictions
- Automate retraining pipeline

### 10.2 Active Learning Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                    ACTIVE LEARNING PIPELINE                      │
└─────────────────────────────────────────────────────────────────┘

Production Inference
        │
        ├── confidence > 0.9 ──────▶ Archive (no action)
        │
        ├── 0.7 < confidence < 0.9 ─▶ Flag for review
        │
        └── confidence < 0.7 ───────▶ Review Queue
                                            │
                                            ▼
                                    ┌───────────────┐
                                    │ Label Studio  │
                                    │ Manual Review │
                                    └───────┬───────┘
                                            │
                                            ▼
                                    ┌───────────────┐
                                    │ Add to        │
                                    │ Training Set  │
                                    └───────┬───────┘
                                            │
                                            ▼
                                    ┌───────────────┐
                                    │ Retrain Model │
                                    │ (Weekly)      │
                                    └───────┬───────┘
                                            │
                                            ▼
                                    ┌───────────────┐
                                    │ Deploy Update │
                                    │ (If improved) │
                                    └───────────────┘
```

### 10.3 Prediction Logger

```python
# scripts/prediction_logger.py
import json
import cv2
from datetime import datetime

class PredictionLogger:
    def __init__(self, log_dir, review_threshold=0.7):
        self.log_dir = log_dir
        self.review_threshold = review_threshold
        
    def log(self, image, detections):
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S_%f")
        
        for det in detections:
            if det['confidence'] < self.review_threshold:
                # Save to review queue
                img_path = f"{self.log_dir}/review_queue/{timestamp}.jpg"
                json_path = f"{self.log_dir}/review_queue/{timestamp}.json"
                
                cv2.imwrite(img_path, image)
                with open(json_path, 'w') as f:
                    json.dump({
                        'timestamp': timestamp,
                        'detection': det,
                        'needs_review': True
                    }, f)
```

### 10.4 Metrics Dashboard

Track the following metrics in Weights & Biases:

| Metric | Target | Description |
|--------|--------|-------------|
| mAP@50 | > 0.85 | Mean Average Precision |
| mAP@50-95 | > 0.65 | Stricter mAP |
| Precision | > 0.90 | True positives / all positives |
| Recall | > 0.85 | True positives / actual positives |
| Inference Time | < 50ms | Per-image latency |

### 10.5 Retraining Trigger Conditions

```python
# Automatic retraining when:
RETRAIN_CONDITIONS = {
    'min_new_samples': 500,      # At least 500 new labeled samples
    'max_days_since_train': 7,   # Maximum 7 days between trainings
    'accuracy_drop_threshold': 0.05  # Retrain if accuracy drops 5%
}
```

---

## 11. Project Timeline

### 11.1 Phase Schedule

| Phase | Duration | Tasks |
|-------|----------|-------|
| **Week 1-2** | Setup | Hardware assembly, environment setup |
| **Week 3-4** | Data Collection | Initial image capture, dataset download |
| **Week 5-6** | Labeling | Label 2,000+ images |
| **Week 7-8** | Training | Train and validate models |
| **Week 9-10** | Deployment | TensorRT conversion, edge deployment |
| **Week 11-12** | Testing | Integration testing, accuracy tuning |
| **Ongoing** | Active Learning | Continuous improvement |

### 11.2 Milestones

| Milestone | Target Date | Criteria |
|-----------|-------------|----------|
| M1: Hardware Ready | Week 2 | Jetson + camera operational |
| M2: Data Pipeline | Week 4 | S3 sync working |
| M3: Initial Model | Week 8 | mAP > 0.70 |
| M4: Deployed | Week 10 | Real-time inference working |
| M5: Production | Week 12 | mAP > 0.85, < 50ms latency |

---

## 12. File Structure

### 12.1 Project Repository

```
waste-detector/
├── README.md
├── requirements.txt
├── setup.py
│
├── config/
│   ├── waste_taxonomy.yaml          # Complete taxonomy
│   ├── yolo_training_config.yaml    # YOLO training config
│   ├── label_studio_config.xml      # Label Studio template
│   └── device_config.yaml           # Jetson settings
│
├── scripts/
│   ├── merge_datasets.py            # Dataset merger
│   ├── capture.py                   # Image capture
│   ├── s3_sync.py                   # Cloud sync
│   ├── train.py                     # Training script
│   ├── export_tensorrt.py           # Model export
│   └── prediction_logger.py         # Logging
│
├── inference/
│   ├── detector.py                  # Main detector class
│   ├── sorting_logic.py             # Bin determination
│   └── ui_display.py                # Optional touchscreen
│
├── datasets/
│   ├── trashnet/                    # Downloaded dataset
│   ├── taco/                        # Downloaded dataset
│   └── merged/                      # Combined dataset
│       ├── images/
│       │   ├── train/
│       │   ├── val/
│       │   └── test/
│       └── labels/
│           ├── train/
│           ├── val/
│           └── test/
│
├── models/
│   ├── waste_detector.pt            # PyTorch weights
│   ├── waste_detector.onnx          # ONNX export
│   └── waste_detector.engine        # TensorRT engine
│
├── runs/
│   └── train/                       # Training outputs
│
└── tests/
    ├── test_detector.py
    └── test_sorting.py
```

---

## 13. Appendix

### 13.1 Useful Commands

```bash
# Check Jetson stats
sudo tegrastats

# Monitor GPU usage
watch -n 1 nvidia-smi

# Test camera
gst-launch-1.0 nvarguscamerasrc ! nvoverlaysink

# Run inference benchmark
python -c "from ultralytics import YOLO; YOLO('model.engine').benchmark()"
```

### 13.2 Troubleshooting

| Issue | Solution |
|-------|----------|
| Camera not detected | Check CSI ribbon cable connection |
| Out of memory | Reduce batch size or image size |
| Slow inference | Ensure TensorRT engine is built for Orin |
| Poor accuracy | Add more training data, check labels |

### 13.3 References

- [Ultralytics YOLOv8 Documentation](https://docs.ultralytics.com/)
- [NVIDIA Jetson Documentation](https://developer.nvidia.com/embedded/jetson-orin-nano)
- [Label Studio Documentation](https://labelstud.io/guide/)
- [TACO Dataset](http://tacodataset.org/)
- [TrashNet Dataset](https://github.com/garythung/trashnet)
- [ST6 Ameru Case Study](https://st6.io/work/ameru/)

### 13.4 License

This implementation plan is provided for educational purposes. Individual components may have their own licenses:
- TACO Dataset: CC BY 4.0
- TrashNet: MIT License
- YOLOv8: AGPL-3.0

---

## Supporting Files

The following files are included with this implementation plan:

1. **`config/label_studio_config.xml`** - Label Studio annotation configuration
2. **`scripts/merge_datasets.py`** - Python script to merge TrashNet and TACO
3. **`config/yolo_training_config.yaml`** - YOLO training configuration
4. **`config/waste_taxonomy.yaml`** - Complete waste classification taxonomy

---

*Document generated: January 2026*  
*Based on Ameru.AI/ST6 architecture*
