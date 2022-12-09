# Rust_Project

AKKI Yacine, DAROUECHE Salim, KOMBILA Joël, RUAU Nicolas

## Lancement du server
```
cd src/server
```
```
cargo run
```
## Lancement du beacon

```
cd src/beacon
```
```
cargo run
```
## simulation d'un attaquant
connection au server
```
telnet localhost 3333
```
Une fois connecté tapez:
```
admin
```
Cela vous authentifiera comme attaquant, afin de vous différenciez des beacon

### Liste des commandes supportées
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
