# Local LLM Setup for Altair (AMD RX 9070 XT)

> **What this is:** A guide to running AI models locally for task breakdown and knowledge graphs using your AMD GPU

**Time to setup:** 30-45 minutes
**Hardware:** AMD RX 9070 XT (16GB VRAM) + Ryzen 9 9900x

---

## 🎯 Quick Answer

**Best models for your setup:**

| Use Case             | Model                   | Speed     | VRAM   | Why                   |
| -------------------- | ----------------------- | --------- | ------ | --------------------- |
| **Task breakdown**   | Qwen 2.5 14B (Q5_K_M)   | 20-25 t/s | 10GB   | Best JSON reliability |
| **Fast parsing**     | Llama 3.1 8B (Q5_K_M)   | 50-60 t/s | 5GB    | Real-time feedback    |
| **Knowledge graphs** | DeepSeek-R1 8B (Q5_K_M) | 45-60 t/s | 5GB    | Shows reasoning       |
| **Complex planning** | Llama 3.3 70B (Q4_K_M)  | 5-10 t/s  | 42GB\* | Batch processing      |

\*Requires RAM offload

**Recommended combo:** Qwen 2.5 14B + DeepSeek-R1 8B (15GB VRAM total)

---

## 🚀 Quick Setup (30 minutes)

> ⚠️ **AMD GPU Note:** Your RX 9070 XT needs extra configuration - it works but isn't officially supported yet

**Quick Reference - What applies to you:**

| Component      | Ubuntu 25.04       | Arch Linux                | Fish Shell                          | Bash/Zsh             |
| -------------- | ------------------ | ------------------------- | ----------------------------------- | -------------------- |
| ROCm Install   | ✅ Manual          | ✅ Pacman                 | N/A                                 | N/A                  |
| User Groups    | N/A                | **video, render**         | N/A                                 | N/A                  |
| Ollama Install | ✅ Script          | ✅ AUR (`ollama-rocm`)    | N/A                                 | N/A                  |
| Config Method  | Shell vars         | **Systemd override**      | Shell vars                          | Shell vars           |
| Config Syntax  | N/A                | N/A                       | `set -x VAR "value"`                | `export VAR="value"` |
| Apply Config   | `source ~/.bashrc` | `systemctl daemon-reload` | `source ~/.config/fish/config.fish` | `source ~/.bashrc`   |

> 🚨 **Arch users:** You need 3 things: (1) `video,render` groups + **reboot**, (2) systemd override, (3) `ollama-rocm` package

### Step 1: Install Prerequisites

**Ubuntu 25.04:**

```bash
# Install ROCm 6.4.1+
wget https://repo.radeon.com/amdgpu-install/latest/ubuntu/noble/amdgpu-install.deb
sudo apt install ./amdgpu-install.deb
sudo amdgpu-install --usecase=rocm
```

**Arch Linux:**

```bash
# Install ROCm from official repos
sudo pacman -S rocm-hip-sdk rocm-opencl-sdk

# CRITICAL: Add user to video and render groups
sudo usermod -a -G video,render $USER

# Install Ollama with ROCm support
yay -S ollama-rocm  # or: paru -S ollama-rocm

# Enable service (don't start yet - need reboot first)
sudo systemctl enable ollama
```

> ℹ️ **Note:** Ubuntu 22.04/24.04 won't work (kernel too old). Arch Linux has ROCm in official repos!
> ⚠️ **Important:** Use `ollama-rocm` not `ollama` for AMD GPU support
> 🚨 **Critical:** You MUST be in `video` and `render` groups, then **REBOOT** (logout is not sufficient!)
> 🚨 **Critical for Arch:** Shell environment variables don't work with systemd services! See [GPU Discovery Crash](#gpu-discovery-crash-arch-linux) troubleshooting section for systemd override setup.

### Step 2: Configure Environment

> ℹ️ **Note:** These shell configurations are for **interactive use only** (running `ollama` commands in terminal). For **systemd service** (Arch Linux), see [GPU Discovery Crash](#gpu-discovery-crash-arch-linux) troubleshooting.

**Bash/Zsh (create config file):**

```bash
# Add to ~/.bashrc or ~/.zshrc
export HSA_OVERRIDE_GFX_VERSION="12.0.0"  # Critical for RX 9070 XT
export ROCR_VISIBLE_DEVICES=0
export HIP_VISIBLE_DEVICES=0
export OLLAMA_GPU_DRIVER=rocm

# Memory optimization
export OLLAMA_NUM_PARALLEL=2
export OLLAMA_MAX_LOADED_MODELS=2
export OLLAMA_KEEP_ALIVE="10m"
export OLLAMA_FLASH_ATTENTION=1
export OLLAMA_KV_CACHE_TYPE="q8_0"
```

**Apply changes:**

```bash
source ~/.bashrc  # or source ~/.zshrc
```

**Fish shell (create config file):**

```fish
# Add to ~/.config/fish/config.fish
set -x HSA_OVERRIDE_GFX_VERSION "12.0.0"  # Critical for RX 9070 XT
set -x ROCR_VISIBLE_DEVICES 0
set -x HIP_VISIBLE_DEVICES 0
set -x OLLAMA_GPU_DRIVER rocm

# Memory optimization
set -x OLLAMA_NUM_PARALLEL 2
set -x OLLAMA_MAX_LOADED_MODELS 2
set -x OLLAMA_KEEP_ALIVE "10m"
set -x OLLAMA_FLASH_ATTENTION 1
set -x OLLAMA_KV_CACHE_TYPE "q8_0"
```

**Apply changes:**

```fish
source ~/.config/fish/config.fish
```

> 💡 **Tip:** Fish shell uses `set -x` instead of `export`

### Step 3: Install Ollama

```bash
# Ubuntu/Fedora
curl -fsSL https://ollama.com/install.sh | sh

# Arch Linux (use ROCm-specific package for AMD GPUs)
yay -S ollama-rocm
# or: paru -S ollama-rocm
```

**Verify it's running:**

```bash
systemctl status ollama  # Should show "active (running)"
```

> 💡 **Arch Note:** `ollama-rocm` is built specifically for AMD GPUs. Don't use plain `ollama` package!

### Step 4: Pull Models

**For production (both models):**

```bash
ollama pull qwen2.5:14b-instruct-q5_K_M     # Task breakdown (10GB)
ollama pull deepseek-r1:8b  # Knowledge graphs (5GB)
```

**For development (lighter):**

```bash
ollama pull llama3.1:8b-instruct-q5_K_M     # Task parsing (5GB)
ollama pull deepseek-r1:8b  # Knowledge graphs (5GB)
```

### Step 5: Verify Setup

```bash
# Check GPU is recognized
rocm-smi

# Test model
ollama run qwen2.5:14b-instruct-q5_K_M "Break down: Write documentation"
```

**Expected:** GPU% should show 90-99% during inference

> ✅ **Success:** If you see structured output and high GPU usage, you're ready!

---

## 📋 Model Details

### Task Breakdown Models

#### Qwen 2.5 14B (Production Choice)

**When to use:** Main API endpoints for task breakdown

**Strengths:**

- Explicitly optimized for JSON generation
- 79.7 MMLU benchmark (strong reasoning)
- Consistent structured output
- Reliable formatting

**Performance:**

- **Speed:** 20-25 tokens/second
- **VRAM:** 10GB
- **Quality:** Best balance for production

**Install:**

```bash
ollama pull qwen2.5:14b-instruct-q5_K_M
```

#### Llama 3.1 8B (Fast Alternative)

**When to use:** Real-time user feedback, development

**Strengths:**

- Native function calling
- Fast response time
- Built-in tool use
- Lower VRAM

**Performance:**

- **Speed:** 50-60 tokens/second
- **VRAM:** 5GB
- **Quality:** Good for straightforward tasks

**Install:**

```bash
ollama pull llama3.1:8b-instruct-q5_K_M
```

#### Llama 3.3 70B (Complex Planning)

**When to use:** Weekly deep analysis, complex multi-step planning

**Strengths:**

- 92.1 IFEval score (best instruction-following)
- 128K token context window
- Superior semantic understanding

**Performance:**

- **Speed:** 5-10 tokens/second (slow due to CPU offload)
- **VRAM:** 42GB (requires heavy quantization + RAM)
- **Quality:** Best for accuracy-critical operations

**Install:**

```bash
ollama pull llama3.3:70b
```

> 💡 **Tip:** Use this for overnight batch processing, not real-time

### Knowledge Graph Models

#### DeepSeek-R1 8B (Primary Choice)

**When to use:** Finding connections between notes and tasks

**Strengths:**

- Shows reasoning with `<think>` tags
- 90.8% MMLU, 97.3% MATH-500
- Transparent decision-making
- ADHD-friendly (explains why connections exist)

**Performance:**

- **Speed:** 45-60 tokens/second
- **VRAM:** 5GB
- **Quality:** Best reasoning for size

**Install:**

```bash
ollama pull deepseek-r1:8b
```

**Example output:**

```
<think>
Task mentions "API documentation" and Note #42 discusses
"REST endpoint patterns" - these are related because...
</think>

Relationship: Task → Related_To → Note #42
Confidence: 0.85
```

#### Triplex (Specialized Alternative)

**When to use:** Extracting explicit entity relationships

**Strengths:**

- Purpose-built for knowledge graphs
- Outputs (subject, predicate, object) triplets
- 98% cost reduction vs GPT-4 (Microsoft testing)

**Performance:**

- **Speed:** 70-80 tokens/second
- **VRAM:** 3GB
- **Quality:** Specialized but narrow

**Install:**

```bash
ollama pull sciphi/triplex
```

> ℹ️ **Note:** Use as supplement to DeepSeek-R1, not replacement

---

## ⚙️ Configuration

### Production Modelfile (Qwen 2.5)

**Create:** `~/altair-task-breakdown.modelfile`

```modelfile
FROM qwen2.5:14b-instruct-q5_K_M

# Hardware optimization
PARAMETER num_ctx 4096              # Context window
PARAMETER num_batch 512             # Batch processing
PARAMETER num_gpu 40                # All layers to GPU
PARAMETER num_thread 8              # CPU threads

# JSON generation
PARAMETER temperature 0.2           # Deterministic output
PARAMETER top_p 0.8                 # Conservative sampling
PARAMETER top_k 20                  # Narrow token selection
PARAMETER repeat_penalty 1.05       # Light repetition control

SYSTEM """You are an expert task breakdown assistant for ADHD users.
Convert freeform descriptions into clear, actionable steps.
Output valid JSON only, no markdown formatting."""
```

**Load model:**

```bash
ollama create altair-tasks -f ~/altair-task-breakdown.modelfile
```

### Knowledge Graph Modelfile (DeepSeek-R1)

**Create:** `~/altair-knowledge-graph.modelfile`

```modelfile
FROM deepseek-r1:8b

# Hardware optimization
PARAMETER num_ctx 8192              # Larger for multi-doc analysis
PARAMETER num_batch 512
PARAMETER num_gpu 35                # All layers to GPU
PARAMETER num_thread 8

# Reasoning settings
PARAMETER temperature 0.3           # Balanced
PARAMETER top_p 0.9                 # Standard sampling
PARAMETER top_k 40                  # Moderate diversity

SYSTEM """You are a knowledge graph specialist.
Identify meaningful relationships between notes, tasks, and projects.
Show your reasoning process, then output structured relationship data."""
```

**Load model:**

```bash
ollama create altair-graph -f ~/altair-knowledge-graph.modelfile
```

---

## 💻 Usage Examples

### Python Integration (FastAPI Backend)

**Task breakdown with JSON schema:**

```python
from ollama import chat
from pydantic import BaseModel, Field
from typing import List

class ActionStep(BaseModel):
    step_number: int
    description: str
    estimated_duration: str
    energy_level: str  # low, medium, high

class TaskBreakdown(BaseModel):
    task_summary: str
    total_time: str
    steps: List[ActionStep]
    adhd_tips: str

# Use structured output
response = chat(
    model='altair-tasks',
    format=TaskBreakdown.model_json_schema(),
    messages=[{
        'role': 'user',
        'content': f'Break down: {user_input}'
    }],
    options={'temperature': 0.2}
)

result = TaskBreakdown.model_validate_json(response.message.content)
```

**Knowledge graph relationships:**

```python
class Relationship(BaseModel):
    source: str
    target: str
    type: str  # depends_on, relates_to, blocks, enables
    confidence: float  # 0-1
    reasoning: str

class GraphUpdate(BaseModel):
    relationships: List[Relationship]

# Extract with reasoning
response = chat(
    model='altair-graph',
    format=GraphUpdate.model_json_schema(),
    messages=[{
        'role': 'user',
        'content': f'Find connections: {notes_context}'
    }],
    options={'temperature': 0.3}
)

update = GraphUpdate.model_validate_json(response.message.content)
```

---

## 📊 Performance Expectations

### Speed Benchmarks (RX 9070 XT)

| Model Size | Quantization | Speed (t/s) | VRAM   | Quality |
| ---------- | ------------ | ----------- | ------ | ------- |
| 7-8B       | Q5_K_M       | 50-65       | 5GB    | 98.2%   |
| 7-8B       | Q4_K_M       | 60-75       | 4GB    | 97.1%   |
| 13-14B     | Q5_K_M       | 20-30       | 10GB   | 98.2%   |
| 13-14B     | Q4_K_M       | 25-35       | 8GB    | 97.1%   |
| 70B        | Q4_K_M       | 5-10        | 42GB\* | 97.1%   |

\*CPU offload required

### AMD vs NVIDIA Performance

**Reality check:** AMD GPUs run at 50-70% of equivalent NVIDIA speed

**Your RX 9070 XT ≈ RTX 4070** in speed, but with:

- ✅ More VRAM (16GB vs 12GB)
- ✅ Better value for money
- ❌ More setup complexity
- ❌ Less official support

> 💡 **Tip:** Humans read ~5 tokens/second, so 20-25 t/s is plenty fast

---

## 🎯 Quantization Guide

**What is quantization?** Compressing model size with minimal quality loss

### Recommended Levels

**Q5_K_M (Best Balance):**

- **Quality:** 98.2% of original
- **VRAM:** ~5GB per 8B model
- **Use for:** Production

**Q4_K_M (More Capacity):**

- **Quality:** 97.1% of original
- **VRAM:** ~4GB per 8B model
- **Use for:** Running multiple models

**Q8_0 (Highest Quality):**

- **Quality:** 99.5% of original
- **VRAM:** ~7GB per 8B model
- **Use for:** When you have VRAM to spare

### ❌ Avoid

**Q2_K, Q3_K:** Unpredictable behavior, significant quality loss

### Memory Footprint Reference

```
8B models:
├─ Q4_K_M → 4.6GB VRAM
├─ Q5_K_M → 5.0GB VRAM
└─ Q8_0   → 6.7GB VRAM

14B models:
├─ Q4_K_M → 8GB VRAM
└─ Q5_K_M → 10GB VRAM

70B models:
└─ Q4_K_M → 42GB VRAM (requires RAM offload)
```

---

## 🔧 Deployment Strategy

### Development Setup

**Lightweight combo (10GB VRAM):**

```bash
# Fast iteration, plenty of memory headroom
ollama pull llama3.1:8b-instruct-q5_K_M
ollama pull deepseek-r1:8b
```

**Benefits:**

- Fast hot reload
- Room for debugging tools
- Quick response times
- 6GB VRAM free

### Production Setup

**Optimal combo (15GB VRAM):**

```bash
# Best quality, production-ready
ollama pull qwen2.5:14b-instruct-q5_K_M
ollama pull deepseek-r1:8b
```

**Benefits:**

- Reliable JSON output
- Strong reasoning
- 1GB VRAM headroom
- 46-47GB system RAM for PostgreSQL/FastAPI

### Multi-Model Architecture

**Run both models simultaneously:**

```bash
# Start Ollama service
ollama serve

# Models auto-load on first request
# Both stay in VRAM for 10 minutes (OLLAMA_KEEP_ALIVE)
```

**FastAPI endpoints:**

```python
# Task breakdown endpoint
@app.post("/api/tasks/breakdown")
async def breakdown_task(task: str):
    return await ollama.chat(model='altair-tasks', ...)

# Knowledge graph endpoint
@app.post("/api/graph/analyze")
async def analyze_connections(notes: List[str]):
    return await ollama.chat(model='altair-graph', ...)
```

---

## 🚨 Troubleshooting

### GPU Discovery Crash (Arch Linux)

**Symptoms:** Ollama service crashes during GPU detection, logs show:

- `"failure during GPU discovery" error="runner crashed"`
- `"entering low vram mode" "total vram"="0 B"`
- Stack trace mentioning `libhsa-runtime64.so.1`
- `offloaded 0/33 layers to GPU` (running on CPU)
- `timed out waiting for llama runner to start`

**Root causes:**

1. Environment variables not passed to systemd service
2. User not in required groups (video/render)

**Fix 1 - Check user groups (CRITICAL):**

```bash
# Check current groups
groups

# Should include: video render
# If missing, add yourself:
sudo usermod -a -G video,render $USER

# REBOOT (required! logout is not sufficient)
sudo reboot

# After reboot, verify:
groups  # Should now show video and render
```

**Fix 2 - Create systemd override:**

```bash
# Create override directory
sudo mkdir -p /etc/systemd/system/ollama.service.d

# Create override file
sudo nano /etc/systemd/system/ollama.service.d/override.conf
```

**Add this content:**

```ini
[Service]
Environment="HSA_OVERRIDE_GFX_VERSION=12.0.0"
Environment="ROCR_VISIBLE_DEVICES=0"
Environment="HIP_VISIBLE_DEVICES=0"
Environment="OLLAMA_GPU_DRIVER=rocm"
Environment="OLLAMA_NUM_PARALLEL=2"
Environment="OLLAMA_MAX_LOADED_MODELS=2"
Environment="OLLAMA_KEEP_ALIVE=10m"
Environment="OLLAMA_FLASH_ATTENTION=1"
Environment="OLLAMA_KV_CACHE_TYPE=q8_0"
```

**Reload and restart:**

```bash
# Reload systemd configuration
sudo systemctl daemon-reload

# Restart Ollama
sudo systemctl restart ollama

# Check status
systemctl status ollama
```

**Verify GPU is working:**

```bash
# Check logs for GPU detection (should NOT crash now)
journalctl -u ollama -f

# In another terminal, test model
ollama run llama3.1:8b-instruct-q5_K_M "test"

# GPU should show activity
rocm-smi
```

**Expected in logs:**

```
level=INFO source=types.go:129 msg="inference compute" id=GPU-xxx
library=/usr/lib/ollama compute=gfx1100 name="AMD Radeon RX 9070 XT"
description="AMD Radeon RX 9070 XT" total="16.0 GiB" available="15.8 GiB"
```

> ⚠️ **Critical:** Shell environment variables (`~/.bashrc`, `~/.config/fish/config.fish`) don't apply to systemd services. You MUST use the override file!

### GPU Not Recognized

**Symptoms:** Slow performance, CPU usage high, GPU% shows 0%

**Check:**

```bash
rocm-smi  # Should show RX 9070 XT
echo $HSA_OVERRIDE_GFX_VERSION  # Should be "12.0.0"

# Fish shell:
echo $HSA_OVERRIDE_GFX_VERSION  # Should be "12.0.0"
```

**Fix (Bash/Zsh):**

```bash
# Add to ~/.bashrc or ~/.zshrc
export HSA_OVERRIDE_GFX_VERSION="12.0.0"
source ~/.bashrc

# Restart Ollama
sudo systemctl restart ollama
```

**Fix (Fish shell):**

```fish
# Add to ~/.config/fish/config.fish
set -x HSA_OVERRIDE_GFX_VERSION "12.0.0"
source ~/.config/fish/config.fish

# Restart Ollama
sudo systemctl restart ollama
```

**Fix (Arch Linux with systemd service):**
See [GPU Discovery Crash](#gpu-discovery-crash-arch-linux) section above for systemd override configuration.

> 💡 **Fish Tip:** Use `set -x` instead of `export` - fish uses different syntax
> 🚨 **Arch Tip:** Shell environment variables don't apply to systemd services!

### Kernel/Mesa Too Old

**Symptoms:** GPU not detected, rocm-smi fails

**Solution - Ubuntu:** Upgrade to:

- Ubuntu 25.04 (not 24.04)
- Kernel 6.8+
- Mesa 25+

**Solution - Arch Linux:**

```bash
# Arch usually has latest kernel/Mesa
sudo pacman -Syu  # Update everything

# Verify versions
uname -r  # Should be 6.8+
pacman -Q mesa  # Should be 25+

# If outdated, ensure you're not on LTS kernel
sudo pacman -S linux linux-headers  # Install latest kernel
```

**Solution - Fedora:**

```bash
sudo dnf update
# Fedora typically ships with recent kernels
```

### Arch Linux: ROCm Installation Issues

**Symptoms:** `rocm-smi` command not found, ROCm packages missing

**Fix:**

```bash
# Ensure ROCm packages are installed
sudo pacman -S rocm-hip-sdk rocm-opencl-sdk rocm-smi-lib

# Add user to video and render groups
sudo usermod -a -G video,render $USER

# Reboot for group changes to take effect
sudo reboot
```

**After reboot, verify installation:**

```bash
rocm-smi  # Should show GPU info
rocminfo | grep "Name:"  # Should list RX 9070 XT
groups  # Should show: video render
```

### Arch Linux: Wrong Ollama Package

**Symptoms:** GPU not being used, slow performance despite ROCm installed

**Check which package you have:**

```bash
pacman -Q | grep ollama
```

**If you see `ollama` instead of `ollama-rocm`:**

```bash
# Remove wrong package
yay -R ollama

# Install correct package
yay -S ollama-rocm

# Restart service
sudo systemctl restart ollama
```

**Verify GPU usage:**

```bash
rocm-smi  # Should show GPU activity during inference
ollama run qwen2.5:14b-instruct-q5_K_M "test"  # GPU% should spike
```

### Model Running on CPU (0/33 Layers Offloaded)

**Symptoms:**

- Logs show `offloaded 0/33 layers to GPU`
- `timed out waiting for llama runner to start`
- Slow inference speed (CPU-bound)

**Root causes:**

1. Missing user group membership (most common)
2. ROCm not detecting GPU properly

**Fix - Check groups (try this first):**

```bash
# Verify you're in required groups
groups  # Should show: video render

# If missing either group:
sudo usermod -a -G video,render $USER

# CRITICAL: Reboot (logout is not sufficient for systemd services)
sudo reboot

# After reboot, test:
ollama run llama3.1:8b "test"

# Check offload
journalctl -u ollama -n 20 | grep offload
# Should now show: offloaded 33/33 layers to GPU
```

**Fix - Force GPU with Modelfile:**

```bash
# Create modelfile
cat > ~/llama-gpu.modelfile << 'EOF'
FROM llama3.1:8b
PARAMETER num_gpu 999
EOF

# Create custom model
ollama create llama3.1-gpu -f ~/llama-gpu.modelfile

# Test
ollama run llama3.1-gpu "test"
```

**Verify it worked:**

```bash
# Check offload in logs
journalctl -u ollama -n 20 | grep offload

# Watch GPU usage
watch -n 1 rocm-smi  # Should show memory usage and activity
```

### Out of Memory

**Symptoms:** Model crashes, VRAM errors

**Check usage:**

```bash
rocm-smi  # See VRAM usage
ollama ps  # See loaded models
```

**Fix:**

```bash
# Reduce models in VRAM
export OLLAMA_MAX_LOADED_MODELS=1

# Use lighter quantization
ollama pull qwen2.5:14b-q4_k_m  # 8GB instead of 10GB

# Reduce context
# In modelfile: PARAMETER num_ctx 2048
```

### Slow Performance

**Expected speeds:**

- 8B models: 50-60 tokens/second
- 14B models: 20-30 tokens/second

**If slower:**

1. **Check GPU usage:**

```bash
watch -n 1 rocm-smi  # Should show 90-99% GPU during inference
```

1. **Verify num_gpu setting:**

```modelfile
# In modelfile, should match model layers:
PARAMETER num_gpu 35  # For 8B models
PARAMETER num_gpu 40  # For 14B models
```

1. **Check ROCm version:**

```bash
rocminfo | grep "ROCm"  # Should be 6.4.1+
```

### JSON Parsing Errors

**Symptoms:** Invalid JSON, inconsistent format

**Fix:**

```python
# Use structured output instead of format="json"
response = chat(
    model='altair-tasks',
    format=YourPydanticModel.model_json_schema(),  # Enforces schema
    messages=[...],
    options={'temperature': 0.2}  # Lower = more consistent
)
```

**Validate output:**

```python
try:
    result = YourModel.model_validate_json(response.message.content)
except ValidationError as e:
    # Retry with temperature 0.1
    pass
```

---

## ✅ Complete Arch Linux Setup Checklist

**If you're starting fresh on Arch, follow this exact sequence:**

```bash
# 1. Install ROCm
sudo pacman -S rocm-hip-sdk rocm-opencl-sdk rocm-smi-lib

# 2. Add user to groups
sudo usermod -a -G video,render $USER

# 3. Install ollama-rocm (not ollama!)
yay -S ollama-rocm
sudo systemctl enable ollama

# 4. Create systemd override
sudo mkdir -p /etc/systemd/system/ollama.service.d
sudo tee /etc/systemd/system/ollama.service.d/override.conf > /dev/null << 'EOF'
[Service]
Environment="HSA_OVERRIDE_GFX_VERSION=12.0.0"
Environment="ROCR_VISIBLE_DEVICES=0"
Environment="HIP_VISIBLE_DEVICES=0"
Environment="OLLAMA_GPU_DRIVER=rocm"
Environment="OLLAMA_NUM_PARALLEL=2"
Environment="OLLAMA_MAX_LOADED_MODELS=2"
Environment="OLLAMA_KEEP_ALIVE=10m"
Environment="OLLAMA_FLASH_ATTENTION=1"
Environment="OLLAMA_KV_CACHE_TYPE=q8_0"
EOF

# 5. REBOOT (critical - logout is not sufficient!)
sudo reboot
```

**After reboot, verify:**

```bash
# Check groups
groups  # Should show: video render

# Start Ollama
sudo systemctl start ollama

# Check status
systemctl status ollama  # Should be "active (running)"

# Check GPU detection
journalctl -u ollama -n 50 | grep -i "inference compute"
# Should show: description="AMD Radeon RX 9070 XT" compute=gfx1100

# Pull a model
ollama pull llama3.1:8b

# Test it
ollama run llama3.1:8b "test"

# Verify GPU usage
journalctl -u ollama -n 20 | grep offload
# Should show: offloaded 33/33 layers to GPU

# Watch GPU
rocm-smi  # Should show memory usage and activity
```

**If any step fails, see the troubleshooting sections above.**

---

## 📈 Monitoring

### Health Check Commands

**System overview:**

```bash
# GPU status
rocm-smi

# Loaded models
ollama ps

# System resources
htop
```

**Performance benchmark:**

```bash
time ollama run qwen2.5:14b-instruct-q5_K_M "Break down: Write API docs"
```

**Expected output:**

```
Tokens per second: 22.5
GPU Memory: 10.2GB / 16GB
CPU Memory: 2.3GB
```

### Production Monitoring

**Log important metrics:**

- Token generation speed
- VRAM usage per request
- Response time
- GPU utilization %

**Alert on:**

- GPU% < 80% (possible CPU fallback)
- VRAM > 15GB (approaching limit)
- Speed < 15 t/s for 14B (performance issue)

---

## 🎓 Next Steps

**You're ready to:**

- [ ] Integrate with FastAPI backend
- [ ] Test task breakdown with real data
- [ ] Build knowledge graph connections
- [ ] Set up batch processing for Llama 3.3 70B
- [ ] Monitor performance in production

**Recommended order:**

1. Get Qwen 2.5 working for task breakdown
2. Add DeepSeek-R1 for knowledge graphs
3. Build FastAPI integration
4. Optimize based on real usage patterns
5. Add Llama 3.3 70B for complex analysis (optional)

---

## 📚 Reference

### Key Environment Variables

**Bash/Zsh:**

```bash
# AMD GPU Support
HSA_OVERRIDE_GFX_VERSION="12.0.0"  # Critical for RX 9070 XT
ROCR_VISIBLE_DEVICES=0
HIP_VISIBLE_DEVICES=0

# Memory Management
OLLAMA_NUM_PARALLEL=2           # Parallel requests per model
OLLAMA_MAX_LOADED_MODELS=2      # Models in VRAM simultaneously
OLLAMA_KEEP_ALIVE="10m"         # Auto-unload after 10 min idle

# Optimization
OLLAMA_FLASH_ATTENTION=1        # For long contexts
OLLAMA_KV_CACHE_TYPE="q8_0"    # 50% KV cache reduction
```

**Fish shell:**

```fish
# AMD GPU Support
set -x HSA_OVERRIDE_GFX_VERSION "12.0.0"  # Critical for RX 9070 XT
set -x ROCR_VISIBLE_DEVICES 0
set -x HIP_VISIBLE_DEVICES 0

# Memory Management
set -x OLLAMA_NUM_PARALLEL 2           # Parallel requests per model
set -x OLLAMA_MAX_LOADED_MODELS 2      # Models in VRAM simultaneously
set -x OLLAMA_KEEP_ALIVE "10m"         # Auto-unload after 10 min idle

# Optimization
set -x OLLAMA_FLASH_ATTENTION 1        # For long contexts
set -x OLLAMA_KV_CACHE_TYPE "q8_0"    # 50% KV cache reduction
```

### Model Selection Matrix

| Scenario             | Primary Model     | Secondary Model     | Total VRAM |
| -------------------- | ----------------- | ------------------- | ---------- |
| **Development**      | Llama 3.1 8B      | DeepSeek-R1 8B      | 10GB       |
| **Production**       | Qwen 2.5 14B      | DeepSeek-R1 8B      | 15GB       |
| **High Quality**     | Qwen 2.5 14B (Q8) | DeepSeek-R1 8B (Q8) | 17GB       |
| **Batch Processing** | Llama 3.3 70B     | N/A                 | 42GB\*     |

\*CPU offload required

### Useful Commands

**Ollama management:**

```bash
# List available models
ollama list

# Pull specific quantization
ollama pull qwen2.5:14b-instruct-q5_K_M

# Run interactive chat
ollama run altair-tasks

# Check GPU usage
watch -n 1 rocm-smi

# Delete model
ollama rm model-name

# Update Ollama (Ubuntu/Fedora)
curl -fsSL https://ollama.com/install.sh | sh

# Update Ollama (Arch Linux)
yay -Syu ollama-rocm
# or: paru -Syu ollama-rocm
```

**Arch Linux service management:**

```bash
# Start Ollama service
sudo systemctl start ollama

# Stop Ollama service
sudo systemctl stop ollama

# Restart Ollama
sudo systemctl restart ollama

# Check service status
systemctl status ollama

# View service logs
journalctl -u ollama -f

# View service configuration (including overrides)
systemctl cat ollama

# Check if environment variables are loaded
systemctl show ollama | grep Environment
```

**Fish shell specific:**

```fish
# Check environment variable
echo $HSA_OVERRIDE_GFX_VERSION

# Set temporary variable (current session only)
set -x TEMP_VAR "value"

# Unset variable
set -e VARIABLE_NAME

# List all environment variables
env

# Edit fish config
nano ~/.config/fish/config.fish
# or: vim ~/.config/fish/config.fish

# Reload fish config
source ~/.config/fish/config.fish
```

---

**Questions?** Check the [Ollama documentation](https://ollama.com/docs) or [AMD ROCm docs](https://rocm.docs.amd.com/)

**Found an issue?** File it in the Altair GitHub repo: `github.com/getaltair/altair`
