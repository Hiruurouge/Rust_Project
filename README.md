# Rust_Project

##Lancement du server
```
cd src/server
```
```
cargo run
```
##Lancement du beacon

```
cd src/beacon
```
```
cargo run
```
##simulation d'un attaquant
connection au server
```
telnet localhost 3333
```
###Liste des commandes supporter
La syntaxe est rigide sinon les commandes ne seront pas comprise
S'identifier en tant qu'admin(obligatoire pour attaquant)
```
admin
```
Afficher la liste des ip des beacons actifs
```
list
```
Lancer une commande sur un beacon actif
```
command:<insérer commande bash> target:<ip de la victime>
```
Endormir le beacon
```
sleep
```
Se déconnecter
```
exit
```
