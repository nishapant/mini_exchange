# -*- mode: ruby -*-
# vi: set ft=ruby :

# All Vagrant configuration is done below. The "2" in Vagrant.configure
# configures the configuration version (we support older styles for
# backwards compatibility). Please don't change it unless you know what
# you're doing.
Vagrant.configure("2") do |config|
    # The most common configuration options are documented and commented below.
    # For a complete reference, please see the online documentation at
    # https://docs.vagrantup.com.
  
    # Every Vagrant development environment requires a box. You can search for
    # boxes at https://vagrantcloud.com/search.
    config.vm.provider "virtualbox"
    #config.vm.box = "fin566_ss_base"
    #config.ssh.password = "vagrant"
  
    # Disable automatic box update checking. If you disable this, then
    # boxes will only be checked for updates when the user runs
    # `vagrant box outdated`. This is not recommended.
    # config.vm.box_check_update = false
  
    # Create a forwarded port mapping which allows access to a specific port
    # within the machine from a port on the host machine. In the example below,
    # accessing "localhost:8080" will access port 80 on the guest machine.
    # NOTE: This will enable public access to the opened port
    # config.vm.network "forwarded_port", guest: 80, host: 8080
  
    # Create a forwarded port mapping which allows access to a specific port
    # within the machine from a port on the host machine and only allow access
    # via 127.0.0.1 to disable public access
    # config.vm.network "forwarded_port", guest: 80, host: 8080, host_ip: "127.0.0.1"
  
    # Create a private network, which allows host-only access to the machine
    # using a specific IP.
    # config.vm.network "private_network", ip: "192.168.33.10"
  
    # Create a public network, which generally matched to bridged network.
    # Bridged networks make the machine appear as another physical device on
    # your network.
    # config.vm.network "public_network"
  
    # Share an additional folder to the guest VM. The first argument is
    # the path on the host to the actual folder. The second argument is
    # the path on the guest to mount the folder. And the optional third
    # argument is a set of non-required options.
    # config.vm.synced_folder "../data", "/vagrant_data"
  
    # Provider-specific configuration so you can fine-tune various
    # backing providers for Vagrant. These expose provider-specific options.
    # Example for VirtualBox:
    #
    # config.vm.provider "virtualbox" do |vb|
    #   # Display the VirtualBox GUI when booting the machine
    #   vb.gui = true
    #
    #   # Customize the amount of memory on the VM:
    #   vb.memory = "1024"
    # end
    #
    # View the documentation for the provider you are using for more
    # information on available options.
  
    # Enable provisioning with a shell script. Additional provisioners such as
    # Ansible, Chef, Docker, Puppet and Salt are also available. Please see the
    # documentation for more information about their specific syntax and use.
    # config.vm.provision "shell", inline: <<-SHELL
    #   apt-get update
    #   apt-get install -y apache2
    # SHELL
    #
   
    #################
    #
    #Use the same basic config and base box for all the VMs
  
    config.vm.box = "centos7_mini"
    config.vm.box_url = "https://davidl.web.engr.illinois.edu/vms/centos7_mini.box"
    config.vm.box_download_checksum = "4897db055b26323d6d8d4a3f14b9c7d5a5e770e5d4c0185c0d2915c832719a1f"
    config.vm.box_download_checksum_type = "sha256"
    config.ssh.insert_key = false
  
  
    
    config.vm.define "ome" do |tcp1|
      tcp1.vm.hostname = "ome"
      
      tcp1.vm.provider :virtualbox do |vb|
        vb.customize ["modifyvm", :id, "--memory", "256"]
        vb.customize ["modifyvm", :id, "--cpus", "2"]
      end
    
      tcp1.vm.network "private_network", ip: "192.168.50.101", virtualbox__intnet: "tcp_network", nic_type: "virtio"
    
      tcp1.vm.provision "shell", path: "provision_script/install_sfnettest.sh" #what does this line do?
    end
  
    config.vm.define "esb" do |tcp2|
      tcp2.vm.hostname = "esb"
      
      tcp2.vm.provider :virtualbox do |vb|
        vb.customize ["modifyvm", :id, "--memory", "256"]
        vb.customize ["modifyvm", :id, "--cpus", "2"]
      end
      
      tcp2.vm.network "private_network", ip: "192.168.50.102", virtualbox__intnet: "tcp_network", nic_type: "virtio"
    end
  
    config.vm.define "dropcopy" do |tcp3|
        tcp3.vm.hostname = "dropcopy"
  
        tcp3.vm.provider :virtualbox do |vb|
          vb.customize ["modifyvm", :id, "--memory", "256"]
          vb.customize ["modifyvm", :id, "--cpus", "2"]
        end
  
        tcp3.vm.network "private_network", ip: "192.168.50.103", virtualbox__intnet: "tcp_network", nic_type: "virtio"
    end
  
    config.vm.define "tickerplant" do |tcp4|
          tcp4.vm.hostname = "tickerplant"
  
          tcp4.vm.provider :virtualbox do |vb|
            vb.customize ["modifyvm", :id, "--memory", "256"]
            vb.customize ["modifyvm", :id, "--cpus", "2"]
          end
  
          tcp4.vm.network "private_network", ip: "192.168.50.104", virtualbox__intnet: "tcp_network", nic_type: "virtio"
      end
  
  
    config.vm.define "gateway" do |tcp5|
            tcp5.vm.hostname = "gateway"
  
            tcp5.vm.provider :virtualbox do |vb|
              vb.customize ["modifyvm", :id, "--memory", "256"]
              vb.customize ["modifyvm", :id, "--cpus", "2"]
            end
  
            tcp5.vm.network "private_network", ip: "192.168.50.105", virtualbox__intnet: "tcp_network", nic_type: "virtio"
    end
  
    config.vm.define "trader1" do |tcp6|
            tcp6.vm.hostname = "trader1"
  
            tcp6.vm.provider :virtualbox do |vb|
              vb.customize ["modifyvm", :id, "--memory", "256"]
              vb.customize ["modifyvm", :id, "--cpus", "2"]
            end
  
            tcp6.vm.network "private_network", ip: "192.168.50.106", virtualbox__intnet: "tcp_network", nic_type: "virtio"
    end
  
    config.vm.define "trader2" do |tcp7|
            tcp7.vm.hostname = "trader2"
  
            tcp7.vm.provider :virtualbox do |vb|
              vb.customize ["modifyvm", :id, "--memory", "256"]
              vb.customize ["modifyvm", :id, "--cpus", "2"]
            end
  
            tcp7.vm.network "private_network", ip: "192.168.50.107", virtualbox__intnet: "tcp_network", nic_type: "virtio"
    end
  
    config.vm.define "trader3" do |tcp8|
            tcp8.vm.hostname = "trader3"
  
            tcp8.vm.provider :virtualbox do |vb|
              vb.customize ["modifyvm", :id, "--memory", "256"]
              vb.customize ["modifyvm", :id, "--cpus", "2"]
            end
  
            tcp8.vm.network "private_network", ip: "192.168.50.108", virtualbox__intnet: "tcp_network", nic_type: "virtio"
    end
  
  
  end

  config.vm.define "trader3" do |tcp8|
          tcp8.vm.hostname = "trader3"

          tcp8.vm.provider :virtualbox do |vb|
            vb.customize ["modifyvm", :id, "--memory", "256"]
            vb.customize ["modifyvm", :id, "--cpus", "2"]
          end

          tcp8.vm.network "private_network", ip: "192.168.50.108", virtualbox__intnet: "tcp_network", nic_type: "virtio"
  end


end
  
