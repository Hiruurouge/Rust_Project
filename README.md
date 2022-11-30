# Rust_Project

 Actuellement le serveur ne gère que l'envoie de commande à d'autre machine.
 Pour ça on s'y connectera via deux machine (la deuxieme simuler par un containeur docker ubuntu)UI
 
 ### Mise en place du container pour tester
 ```
 docker run -it --entrypoint "/bin/bash" ubuntu:20.04
 ```
 ```
 apt-get update
 ```
```
apt-get install telnetd
```
```
apt-get install telnet
```

## Lancement du server
 ```
 git clone
 ```
 ```
 cargo run
 ```
 ###utilisation du server
 Dans votre docker (simuler un beacon)
 ```
 telnet votre-ip-local 333
 ```
 Dans un cmd lancez la commande (simuler un attaquant)
 ```
 telnet localhost 3333
 ```
 Afin de se connecter au server
 ### Envoie de commande au beacon
 La syntaxe est rigide.
 Pour envoyer une commande au beacone depuis le cmd attaquant
 ici un exemple
 ```
 command: ls target:10.42.0.1
 ```
 (on envoie la commande "ls" au beacone d'ip 10.42.0.1
### Liste des commandes disponible
```
exit
``` 
pour shutdown une socket
```
list
```
Pour avoir la liste des beacons