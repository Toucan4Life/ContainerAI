# ContainerAI

A digital version of the board game **Container**, built to:

1. Play the game through a UI, including against AI opponents.
2. Train AI agents to play it.

## Stack

- **Game engine:** Rust, for fast simulation during training.
- **Training:** Python with PyTorch (CUDA).
- **UI:** to be decided.

## Development

The project is developed in a devcontainer (Rust, Python 3.12, Node, PyTorch with CUDA).

Requirements on the host:

- Docker
- An NVIDIA GPU with the driver and the [NVIDIA Container Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html)

Open the folder in VS Code and choose **Reopen in Container**. After setup, check the GPU:

```sh
python -c "import torch; print(torch.cuda.is_available())"
```
