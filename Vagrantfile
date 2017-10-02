# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  config.vm.box = "debian/stretch64"
  config.vm.synced_folder ".", "/vagrant", type: "nfs", nfs_version: 3

  config.vm.provider :libvirt do |libvirt|
    libvirt.cpus = 2
    libvirt.nested = true
  end

  config.vm.provision "shell", privileged: false, inline: <<-SHELL
    sudo apt-get update
    sudo apt-get install -y curl
    sudo apt-get install -y linux-headers-\$(uname -r) gcc make git vim
    curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=nightly -y;
    git clone $CLONE_URL
    source ~/.cargo/env

    echo "source ~/.cargo/env" >> ~/.bashrc
    echo "alias v=vim" >> ~/.bashrc
    echo "alias gs='git status'" >> ~/.bashrc
    echo "alias gap='git add -p'" >> ~/.bashrc

    echo "set spell" >> ~/.vimrc
    echo "syntax on" >> ~/.vimrc
    echo "set number" >> ~/.vimrc
    echo "au BufRead,BufNewFile *.rs setfiletype rust" >> ~/.vimrc
    echo "autocmd BufRead,BufNewFile *.rs setlocal expandtab syntax=rust" >> ~/.vimrc
    mkdir -p ~/.vim/syntax

    rustup install nightly
    rustup default nightly
    rustup component add rust-src
    cargo install xargo
  SHELL
end
