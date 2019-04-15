# Virtual machine on internet

I want the game to be played by more people than me and my girlfriend.  
Even if it is just a project in development.  
For that I need a websocket server on the internet.  
I decided to use Azure, because they give one year free account for learning.  
I hope I understand this correctly.  
Most of the big cloud providers have some sort of free trial period.  

## Azure
First I signed up for a free account.  
https://portal.azure.com  
They want to know the credit card number and phone number for verification.  
But they promise they will not charge me untill I explicitly decide to pay.  

## Virtual machine
On the Azure portal home page I clicked Virtual machine and then Add.   
https://portal.azure.com/#home  
I changed the default recommanded VM type and choose the cheapest option B1s.
It is a free trial, but for some services I have to pay with a credit they give me - around $200. So I want to spend it the slowest possible.  
I choose the Linux Ubuntu server 18. I don't know why they don't have Debian.  
They offer 2 Authentication types: password or SSH.  
I can change between then whenever I need with the Reset pasword function.  
If I want to use Azure Serial Console from the Portal website I have to have a password authentication. This works always, even if there are some network problems because it connects over the serial port COM1. Clever.  
But usually I want to connect over SSH from my local computer. Then I have to choose SSH authentication.  
I wanted to know the Virtual machine IP address to produce the SSH key. And this is known only after the VM is created. Therefore I first choosed password authentication.  
I choose to have this 3 ports opened: http 80, https 443 and SSH 22.  
I finished the VM creation and copied the VM IP address.  

### SSH keys
In the Azure portal I go to the created virtual machine and click on Reset Password.  
Choose SSH authentication, write my name and then paste the SSH key I created with this instructions:  
On my win10 machine I have the Linux SubSystem with Debian. I will use bash to work with SSH keys. Now I know the IP of the VM and I can use it to create my SSH key:    
`ssh-keygen -t rsa -b 2048 -C "Luciano@23.101.23.150"`  
The ssh_keygen then asks for the filename and I choose the same name for no obvious reason at all:  
`Luciano@23.101.23.150`  
The passphrase it asks can be simple. 
I need to copy the content of the public key. To show the content of the file with public key:  
`cat /home/luciano/.ssh/Luciano@23.101.23.150.pub`   
I copy the result to clipboard and paste it into the Azure Portal field SSH public key.  
The text should start with "ssh-rsa" and finish with "Luciano@23.101.23.150" in my case.  

## connection
From my computer in bash I write   
`ssh -i Luciano@23.101.23.150 Luciano@23.101.23.150 -v`  
and have successfully connected to my Azure VM.  

## TODO
I need a websocket server. But I would like to have the http server on the same ports. How to do that?