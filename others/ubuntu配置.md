## 服务器操作系统配置

- 安装操作系统

- 配置网络文件
  
  - 账号配置
    
    - 检查当前用户：`whoami`
    
    - 切换超级管理员账号：`sudo -i`
    
    - 设置当前用户密码：`passwd`
    
    - 切换之前的账号：`ctrl + d`、`exit`
  
  - 网络配置
    
    - 检查网卡名称：`ip a`
    
    - 编写`netplan`配置文件：`sudo nano /etc/netplan/<网卡名称>.yaml`
      
      ```yaml
      network:
        version: 2
        renderer: networkd
        ethernets:
          ens18:
            dhcp4: no
            addresses:
              - 192.168.1.100/24   # 替换成你规划给虚拟机的静态IP和子网掩码
            routes:
              - to: default
                via: 192.168.1.1     # 替换成你的网关地址
            nameservers:
              addresses:
                - 223.5.5.5          # DNS服务器地址
                - 8.8.8.8
      ```
    
    - 配置文件权限保护：`sudo chmod 600 /etc/netplan/<网卡名称>.yaml`
    
    - 网络配置测试：`sudo netplan try`
    
    - 网络配置应用：`sudo netplan apply`
    
    - 网络连接测试：`ip addr show <网卡名称>`
  
  - SSH网络服务
    
    - 检查SSH网络服务状态：`systemctl status ssh`
    
    - 如果没有SSH网络服务：安装并启用
      
      ```yaml
      sudo apt update
      sudo apt install -y openssh-server
      sudo systemctl enable ssh --now
      sudo systemctl status ssh
      ```
    
    - 编辑SSH网络服务：`sudo nano /etc/ssh/sshd_config`
      
      说明：`PermitRootLogin 允许root登录`、`PasswordAuthentication 密码认证`
    
    - 重启SSH网络服务：`sudo systemctl restart ssh`
  
  - 主机名问题
    
    - 查看主机名称：`hostname`
    
    - 编辑主机名称配置：`sudo nano /etc/hosts`
    
    - 在这一行加：`127.0.0.1   localhost <主机名称>`
  
  - 新的服务器操作系统没有IP网关配置、openssh服务、没有密码配置
  
  - 编写配置文件记得空格
  
  - 配置`root`密码

- 安装`docker`
  
  - 轩辕镜像源一键安装
    
    `bash <(wget -qO- https://xuanyuan.cloud/docker.sh)`
  
  - 是否将当前用户添加进入`docker`用户组：当前用户能不能使用`docker`
  
  - 检查镜像配置：`docker info | grep -A 10 "Registry Mirrors"`
  
  - 如果未成功，则手动添加：`sudo nano /etc/docker/daemon.json`
  
  - 新增配置内容：
    
    ```yaml
    "insecure-registries": ["***.xuanyuan.run"],
    "registry-mirrors": ["https://***.xuanyuan.run"]
    ```
  
  - 重新加载配置文件：`systemctl daemon-reload`
  
  - 重新启动`docker`服务：`systemctl daemon-reload`

- 安装`coolify`
  
  - `curl -fsSL https://cdn.coollabs.io/coolify/install.sh | sudo bash`
  
  - 检查防火墙是否开放端口，或者是否关闭（内网访问）
  
  - 检查服务器本地是否可以访问平台
  
  - 检查公司内部AD是否允许访问，注意别影响公司其他服务器的使用
  
  - 账号：root_admin
  
  - 密码：Lj54572905..
