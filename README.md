# rust-climate (en construction)

Ce programme, écrit dans le langage de programmation [Rust](https://www.rust-lang.org/), est le support de mon TPE.

Il s'agit d'une implémentation minimale du modèle climatique décrit dans [How do Climate Models Work?](http://climatesight.org/2012/01/20/how-do-climate-models-work/), de Kaitlin Alexander.

`rust-climate` s'appuie sur la base de données construite par le [National Renewable Energy Laboratory (NREL)](http://www.nrel.gov/) pour récupérer l'ensoleillement de lieux. Cette base de données est exposée au travers d'une [API](https://developer.nrel.gov/docs/solar/solar-resource-v1/) libre d'accès, permettant de connaître l'intensité du rayonnement solaire à des coordonnées GPS données.

Une fois ces données collectées, le programme calcule le réchauffement solaire d'un matériau choisi par l'utilisateur situé à ces coordonnées.
