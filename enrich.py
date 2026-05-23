import json
import os

def run_enrichment():
    json_path = '/opt/gitspace/fisherman-cli/data/game_data.json'
    src_json_path = '/opt/gitspace/fisherman-cli/src/data/game_data.json'
    
    if not os.path.exists(json_path) and os.path.exists('data/game_data.json'):
        json_path = 'data/game_data.json'
    if not os.path.exists(src_json_path) and os.path.exists('src/data/game_data.json'):
        src_json_path = 'src/data/game_data.json'
    
    # 1. Charger le game_data.json existant
    target_path = None
    if os.path.exists(json_path):
        target_path = json_path
    elif os.path.exists(src_json_path):
        target_path = src_json_path
    else:
        print("Erreur : Aucun fichier game_data.json trouvé !")
        return

    with open(target_path, 'r', encoding='utf-8') as f:
        raw_content = f.read()
    
    # Nettoyer les virgules traînantes du JSON avant de le charger
    import re
    cleaned_content = re.sub(r',(\s*[\]}])', r'\1', raw_content)
    data = json.loads(cleaned_content)

    # Nettoyage des doublons/variantes invalides
    for r_key in ["epic"]:
        if r_key in data.get("fish_data", {}):
            data["fish_data"][r_key] = [f for f in data["fish_data"][r_key] if f["name"] != "Coelacanthe"]
    for r_key in ["common"]:
        if r_key in data.get("fish_data", {}):
            data["fish_data"][r_key] = [f for f in data["fish_data"][r_key] if f["name"] not in ["Eperlan", "Chevesne"]]

    # S'assurer que fish_data est structuré
    if "fish_data" not in data:
        data["fish_data"] = {}
        
    for r in ["common", "uncommon", "rare", "veryrare", "epic", "legendary", "mythical", "divin"]:
        if r not in data["fish_data"]:
            data["fish_data"][r] = []

    # 2. Spécification de tous les poissons AC à intégrer/enrichir
    ac_fishes = [
        # format: (nom, rareté_cible, lieu, prix, période, heures, restriction, taille_str, anecdote)
        ("Anchois", "uncommon", "Océan", 200, "Toute l'année", "4h - 21h", "before_22h", "Petite", "Se déplace en bancs immenses pour échapper aux prédateurs de l'océan."),
        ("Arapaïma", "epic", "Rivière", 10000, "Juin - septembre", "16h - 9h", "after_22h", "Énorme", "Un fossile vivant qui respire de l'air à la surface grâce à une vessie natatoire modifiée."),
        ("Arowana", "epic", "Rivière", 10000, "Juin - septembre", "16h - 9h", "after_22h", "Très grande", "Aussi appelé poisson-dragon, il est un symbole de chance et de prospérité en Asie."),
        ("Ayu", "common", "Rivière", 900, "Juillet - septembre", "Toute la journée", None, "Petite/moyenne", "Surnommé poisson-doux en raison de sa chair au parfum unique de melon et de concombre."),
        ("Bar", "common", "Rivière", 400, "Toute l'année", "Toute la journée", None, "Grande", "Le compagnon incontournable de toutes vos sessions de pêche en rivière !"),
        ("Bar commun", "common", "Océan", 400, "Toute l'année", "Toute la journée", None, "Très grande", "Un poisson côtier très recherché pour sa combativité et sa chair délicate."),
        ("Bichir", "rare", "Rivière", 4000, "Juin - septembre", "21h - 4h", "after_22h", "Grande", "Cette créature préhistorique possède des poumons primitifs lui permettant de ramper hors de l'eau."),
        ("Bouvière", "common", "Rivière", 900, "Novembre - mars", "Toute la journée", None, "Minuscule", "Elle pond ses œufs à l'intérieur des moules d'eau douce pour les protéger."),
        ("Brochet", "rare", "Rivière", 1800, "Septembre - décembre", "Toute la journée", None, "Moyenne/grande", "Un redoutable prédateur embusqué doté d'une mâchoire remplie de dents acérées."),
        ("Calmar", "uncommon", "Océan", 500, "Décembre - août", "Toute la journée", None, "Moyenne", "Il peut projeter un jet d'encre noire pour aveugler ses agresseurs et s'enfuir."),
        ("Carangue grosse tête", "rare", "Océan (ponton)", 4500, "Mai - octobre", "Toute la journée", None, "Très grande", "Un colosse de l'océan connu pour sa force brute phénoménale lors des combats."),
        ("Carassin", "common", "Rivière", 160, "Toute l'année", "Toute la journée", None, "Petite", "Très résistant, il est l'ancêtre sauvage du célèbre poisson rouge."),
        ("Cardeau", "uncommon", "Océan", 800, "Toute l'année", "Toute la journée", None, "Très grande", "Un maître du camouflage plat qui se fond parfaitement dans le sable marin."),
        ("Carpe", "common", "Étang", 300, "Toute l'année", "Toute la journée", None, "Moyenne", "Symbole de persévérance, elle peut vivre plusieurs décennies dans les étangs paisibles."),
        ("Carpe koï", "rare", "Étang", 4000, "Toute l'année", "16h - 9h", "after_22h", "Moyenne", "Ces carpes ornementales japonaises sont admirées pour leurs motifs colorés uniques."),
        ("Chevaine", "common", "Rivière", 200, "Toute l'année", "9h - 16h", "before_22h", "Minuscule", "Un poisson omnivore très méfiant qui peuple nos rivières calmes."),
        ("Chinchard", "common", "Océan", 150, "Toute l'année", "Toute la journée", None, "Petite", "Un poisson grégaire très commun, pêché en grande quantité à travers le monde."),
        ("Clione", "uncommon", "Océan", 1000, "Décembre - mars", "Toute la journée", None, "Minuscule", "Surnommé l'ange des mers, c'est en fait un minuscule prédateur mollusque sans coquille."),
        ("Combattant", "rare", "Rivière", 2500, "Mai - octobre", "9h - 16h", "before_22h", "Petite", "Les mâles de cette espèce sont extrêmement territoriaux et arborent de somptueux voiles."),
        ("Coryphène", "epic", "Océan (ponton)", 6000, "Mai - octobre", "Toute la journée", None, "Très grande", "Aussi appelée Mahi-Mahi, elle est célèbre pour ses couleurs dorées et sa vitesse fulgurante."),
        ("Crabe chinois", "rare", "Rivière", 2000, "Septembre - novembre", "16h - 9h", "after_22h", "Petite", "Reconnaissable à ses pinces recouvertes de fourrure, il migre sur de longues distances."),
        ("Crapet", "common", "Rivière", 180, "Toute l'année", "9h - 16h", "before_22h", "Petite", "Un petit poisson d'eau douce très combatif et coloré, originaire d'Amérique du Nord."),
        ("Cyprin doré", "veryrare", "Étang", 1300, "Toute l'année", "9h - 16h", "before_22h", "Minuscule", "Une variété rare et précieuse sélectionnée pour sa couleur orange vif étincelante."),
        ("Cœlacanthe", "legendary", "Océan (pluie)", 15000, "Toute l'année", "Toute la journée", None, "Énorme", "Un véritable dinosaure vivant, que l'on croyait éteint depuis 65 millions d'années !"),
        ("Dai yu", "legendary", "Rivière (falaise)", 15000, "Décembre - mars", "16h - 9h", "after_22h", "Énorme", "Un poisson légendaire des rivières de montagnes, extrêmement rare et mystérieux."),
        ("Dorado", "legendary", "Rivière", 15000, "Juin - septembre", "4h - 21h", "before_22h", "Très grande", "Surnommé l'or des rivières en raison de ses écailles dorées éblouissantes."),
        ("Ecrevisse", "common", "Étang", 200, "Avril - septembre", "Toute la journée", None, "Petite", "Ce petit crustacé d'eau douce adore se cacher sous les pierres des rivières."),
        ("Éperlan", "common", "Rivière", 320, "Décembre - février", "Toute la journée", None, "Petite", "Un petit poisson argenté qui dégage une odeur caractéristique de concombre frais."),
        ("Esturgeon", "epic", "Embouchure de rivière", 10000, "Septembre - mars", "Toute la journée", None, "Énorme", "Ce géant des eaux est célèbre pour ses œufs précieux, le caviar."),
        ("Fondule barré", "common", "Étang", 300, "Avril - août", "Toute la journée", None, "Minuscule", "Un petit poisson rayé extrêmement tolérant aux variations de salinité."),
        ("Gar", "epic", "Étang", 6000, "Juin - septembre", "16h - 9h", "after_22h", "Énorme", "Un poisson préhistorique doté d'un long bec et d'écailles dures comme des pierres."),
        ("Gobie d'eau douce", "uncommon", "Rivière", 400, "Toute l'année", "16h - 9h", "after_22h", "Petite", "Il utilise ses nageoires pelviennes comme ventouses pour s'accrocher aux rochers."),
        ("Grand requin blanc", "legendary", "Océan", 15000, "Juin - septembre", "16h - 9h", "after_22h", "Énorme (aileron)", "Le plus grand prédateur marin de la planète, doté de plusieurs rangées de dents tranchantes !"),
        ("Grenouille", "common", "Étang", 120, "Mai - août", "Toute la journée", None, "Petite", "Un amphibien bavard qui adore chanter sous la pluie d'été au bord de l'étang."),
        ("Guppy", "uncommon", "Rivière", 1300, "Avril - novembre", "9h - 16h", "before_22h", "Minuscule", "Un minuscule poisson tropical très populaire, célèbre pour ses queues colorées en éventail."),
        ("Hippocampe", "uncommon", "Océan", 1100, "Avril - novembre", "Toute la journée", None, "Minuscule", "Un poisson singulier où ce sont les mâles qui portent les œufs dans leur poche ventrale."),
        ("Ishidai", "epic", "Océan", 5000, "Mars - novembre", "Toute la journée", None, "Moyenne", "Reconnaissable à ses rayures noires et blanches, il possède un bec puissant pour briser les coquillages."),
        ("Limande", "common", "Océan", 300, "Octobre - avril", "Toute la journée", None, "Moyenne", "Un poisson plat camouflé dont les deux yeux sont situés du côté droit de sa tête."),
        ("Loche d'étang", "common", "Rivière", 400, "Mars - mai", "Toute la journée", None, "Petite/moyenne", "Elle est sensible aux variations de pression atmosphérique, d'où son surnom de poisson météo."),
        ("Lune de mer", "veryrare", "Océan", 4000, "Juillet - septembre", "4h - 21h", "before_22h", "Énorme (aileron)", "Aussi appelée Mola Mola, ce géant insolite adore flotter sur le flanc pour bronzer au soleil."),
        ("Macropinna", "legendary", "Océan", 15000, "Toute l'année", "21h - 4h", "after_22h", "Petite", "Un poisson abyssal doté d'une tête transparente à travers laquelle on aperçoit ses yeux tubulaires verts !"),
        ("Marlin bleu", "epic", "Océan (ponton)", 10000, "Juillet - septembre, novembre - avril", "Toute la journée", None, "Énorme", "L'un des poissons les plus rapides de l'océan, célèbre pour son rostre en forme d'épée."),
        ("Murène", "rare", "Océan", 2000, "Août - octobre", "Toute la journée", None, "Fine", "Un prédateur marin serpentiforme qui se cache dans les crevasses rocheuses."),
        ("Murène ruban bleue", "uncommon", "Océan", 600, "Juin - octobre", "Toute la journée", None, "Fine", "Elle change de sexe et de couleur au cours de sa vie, devenant d'un bleu électrique magnifique."),
        ("Napoléon", "epic", "Océan", 10000, "Juillet - août", "4h - 21h", "before_22h", "Énorme", "Ce géant des récifs coralliens arbore une bosse frontale caractéristique rappelant le bicorne de l'empereur."),
        ("Néon bleu", "uncommon", "Rivière", 500, "Avril - novembre", "9h - 16h", "before_22h", "Minuscule", "Un petit poisson scintillant originaire d'Amazonie qui brille d'un éclat bleu et rouge."),
        ("Omble", "rare", "Rivière (falaise)", 3800, "Mars - juin, septembre - novembre", "16h - 9h", "after_22h", "Moyenne/grande", "Un poisson des eaux froides et pures de montagne, cousin du saumon."),
        ("Perche jaune", "common", "Rivière", 300, "Octobre - mars", "Toute la journée", None, "Petite/moyenne", "Reconnaissable à ses rayures sombres sur fond doré, elle adore les eaux calmes."),
        ("Piranha", "rare", "Rivière", 2500, "Juin - septembre", "9h - 16h, 21h - 4h", None, "Petite", "Ce redoutable poisson d'eau douce possède des dents acérées comme des rasoirs."),
        ("Poisson arc-en-ciel", "uncommon", "Rivière", 800, "Mai - octobre", "9h - 16h", "before_22h", "Minuscule", "Ses reflets irisés changent de couleur selon l'angle de la lumière."),
        ("Poisson chirurgien", "uncommon", "Océan", 1000, "Avril - septembre", "Toute la journée", None, "Petite", "Il possède des scalpels tranchants à la base de sa queue pour se défendre."),
        ("Poisson docteur", "rare", "Rivière", 1500, "Mai - septembre", "9h - 16h", "before_22h", "Minuscule", "Célèbre pour grignoter doucement les peaux mortes dans les sources thermales."),
        ("Poisson lanterne", "rare", "Océan", 2500, "Novembre - mars", "16h - 9h", "after_22h", "Grande", "Il utilise un leurre lumineux suspendu au-dessus de sa tête pour attirer ses proies dans l'obscurité."),
        ("Poisson rouge", "veryrare", "Étang", 1300, "Toute l'année", "Toute la journée", None, "Minuscule", "Un compagnon domestique classique qui adore nager paisiblement dans les bassins."),
        ("Poisson-ange", "rare", "Rivière", 3000, "Mai - octobre", "16h - 9h", "after_22h", "Petite", "Un magnifique poisson plat et majestueux, très prisé des aquariophiles."),
        ("Poisson-ballon", "epic", "Océan", 5000, "Novembre - février", "21h - 4h", "after_22h", "Moyenne", "Il se gonfle d'eau ou d'air pour effrayer ses prédateurs, et contient un poison mortel."),
        ("Poisson-clown", "uncommon", "Océan", 650, "Avril - septembre", "Toute la journée", None, "Minuscule", "Il vit en parfaite symbiose au milieu des tentacules urticants des anémones de mer."),
        ("Poisson-papillon", "uncommon", "Océan", 1000, "Avril - septembre", "Toute la journée", None, "Petite", "Ses motifs colorés imitent des yeux pour tromper et désorienter ses prédateurs."),
        ("Poisson-porc-épic", "uncommon", "Océan", 250, "Juillet - septembre", "Toute la journée", None, "Moyenne", "Une variété de poisson-globe recouverte d'épines acérées qui se dressent lorsqu'il se gonfle."),
        ("Poisson-ruban", "epic", "Océan", 9000, "Décembre - mai", "Toute la journée", None, "Énorme", "Un géant serpentiforme argenté qui peut atteindre des longueurs impressionnantes dans les abysses."),
        ("Poisson-scorpion", "uncommon", "Océan", 500, "Avril - novembre", "Toute la journée", None, "Moyenne", "Il se fond parfaitement dans les rochers et possède des épines venimeuses redoutables."),
        ("Raie", "rare", "Océan", 3000, "Août - novembre", "4h - 21h", "before_22h", "Énorme", "Elle glisse gracieusement dans l'eau comme si elle volait avec ses nageoires pectorales."),
        ("Ranchu", "veryrare", "Étang", 4500, "Toute l'année", "9h - 16h", "before_22h", "Petite", "Surnommé le roi des poissons rouges au Japon, il est dépourvu de nageoire dorsale."),
        ("Rémora rayé", "rare", "Océan", 1500, "Juin - septembre", "Toute la journée", None, "Moyenne", "Il utilise sa ventouse crânienne pour s'accrocher aux requins et voyager sans effort."),
        ("Requin marteau", "epic", "Océan", 8000, "Juin - septembre", "16h - 9h", "after_22h", "Énorme (aileron)", "La forme unique de sa tête en marteau lui offre une vision panoramique exceptionnelle à 360 degrés."),
        ("Requin scie", "legendary", "Océan", 12000, "Juin - septembre", "16h - 9h", "after_22h", "Énorme (aileron)", "Son long rostre bordé de dents en scie lui sert à déterrer et couper ses proies."),
        ("Requin-baleine", "legendary", "Océan", 13000, "Juin - septembre", "Toute la journée", None, "Énorme (aileron)", "Le plus grand poisson vivant au monde, un géant pacifique qui se nourrit exclusivement de plancton."),
        ("Saumon", "uncommon", "Embouchure de rivière", 700, "Septembre", "Toute la journée", None, "Grande", "Un voyageur infatigable qui remonte les rivières contre le courant pour aller pondre."),
        ("Saumon masou", "uncommon", "Rivière (falaise)", 1000, "Mars - juin, septembre - novembre", "16h - 9h", "after_22h", "Petite/moyenne", "Un saumon sauvage très agile appréciant particulièrement les torrents de montagne."),
        ("Saumon roi", "rare", "Embouchure de rivière", 1800, "Septembre", "Toute la journée", None, "Énorme", "Le plus grand et le plus noble des saumons du Pacifique, à la chair savoureuse."),
        ("Silure", "uncommon", "Étang", 800, "Mai - octobre", "16h - 9h", "after_22h", "Moyenne", "Ce géant d'eau douce sans écailles utilise ses longs barbillons pour s'orienter dans la vase."),
        ("Têtard", "common", "Étang", 100, "Mars - juillet", "Toute la journée", None, "Minuscule", "Un futur ninja de l'étang qui rêve secrètement de maîtriser les techniques secrètes d'invocation !"),
        ("Tête-de-serpent", "epic", "Étang", 5500, "Juin - août", "9h - 16h", "before_22h", "Grande", "Ce redoutable prédateur peut survivre hors de l'eau pendant plusieurs jours en respirant de l'air."),
        ("Thon", "rare", "Océan (ponton)", 7000, "Novembre - avril", "Toute la journée", None, "Énorme", "Un nageur infatigable capable de traverser des océans entiers à une vitesse impressionnante."),
        ("Thon rouge", "veryrare", "Océan", 11000, "Novembre - avril", "Toute la journée", None, "Énorme", "Un colosse marin ultra-rapide doté d'une régulation thermique unique pour les eaux froides."),
        ("Tilapia", "uncommon", "Rivière", 800, "Juin - octobre", "Toute la journée", None, "Moyenne", "Un poisson robuste et adaptable, élevé depuis l'Antiquité en Égypte."),
        ("Tortue serpentine", "epic", "Rivière", 5000, "Avril - octobre", "21h - 4h", "after_22h", "Grande", "Dotée d'une mâchoire puissante, elle ne doit pas être approchée de trop près !"),
        ("Tortue trionyx", "rare", "Rivière", 3750, "Août - septembre", "16h - 9h", "after_22h", "Grande", "Une tortue d'eau douce insolite au corps mou et au long nez en forme de tuba."),
        ("Truite dorée", "legendary", "Rivière (falaise)", 15000, "Mars - mai, septembre - novembre", "16h - 9h", "after_22h", "Moyenne", "Un joyau scintillant des rivières cristallines, extrêmement recherché."),
        ("Vandoise", "common", "Rivière", 240, "Toute l'année", "16h - 9h", "after_22h", "Moyenne", "Un poisson vif et argenté qui adore gober les insectes à la surface des rivières."),
        ("Vivaneau", "rare", "Océan", 3000, "Toute l'année", "Toute la journée", None, "Grande", "Un poisson côtier apprécié pour sa chair ferme et sa belle couleur rouge orangé."),
    ]

    # Custom Fantasy Creatures
    fantasy_fishes = [
        ("Piranhator", "legendary", "Rivière (falaise)", 25000, "Toute l'année", "22h - 4h", "after_22h", "Grande", "Moitié piranha, moitié machine à tuer, il patrouille dans les eaux tumultueuses avec ses yeux lasers."),
        ("Xénomorphe des Abysses", "mythical", "Océan", 50000, "Décembre - mars", "21h - 5h", "after_22h", "Énorme", "Une monstruosité noire née dans les profondeurs insondables de la mer noire. Son sang acide corrode tous les filets."),
        ("Le Crabe Dictateur", "mythical", "Océan", 45000, "Juin - août", "9h - 18h", "before_22h", "Moyenne", "Il règne sans partage sur les récifs, armé de sa pince autoritaire en uniforme d'amiral.")
    ]

    # 3. Dictionnaire de conversion des mois textuels vers des listes d'entiers
    months_mapping = {
        "Toute l'année": list(range(1, 13)),
        "Juin - septembre": [6, 7, 8, 9],
        "Juillet - septembre": [7, 8, 9],
        "Novembre - mars": [11, 12, 1, 2, 3],
        "Septembre - décembre": [9, 10, 11, 12],
        "Décembre - août": [12, 1, 2, 3, 4, 5, 6, 7, 8],
        "Mai - octobre": [5, 6, 7, 8, 9, 10],
        "Décembre - mars": [12, 1, 2, 3],
        "Avril - septembre": [4, 5, 6, 7, 8, 9],
        "Décembre - février": [12, 1, 2],
        "Septembre - mars": [9, 10, 11, 12, 1, 2, 3],
        "Avril - août": [4, 5, 6, 7, 8],
        "Mai - août": [5, 6, 7, 8],
        "Avril - novembre": [4, 5, 6, 7, 8, 9, 10, 11],
        "Mars - novembre": [3, 4, 5, 6, 7, 8, 9, 10, 11],
        "Octobre - avril": [10, 11, 12, 1, 2, 3, 4],
        "Mars - mai": [3, 4, 5],
        "Juillet - septembre, novembre - avril": [7, 8, 9, 11, 12, 1, 2, 3, 4],
        "Août - octobre": [8, 9, 10],
        "Juin - octobre": [6, 7, 8, 9, 10],
        "Juillet - août": [7, 8],
        "Mars - juin, septembre - novembre": [3, 4, 5, 6, 9, 10, 11],
        "Octobre - mars": [10, 11, 12, 1, 2, 3],
        "Novembre - février": [11, 12, 1, 2],
        "Décembre - mai": [12, 1, 2, 3, 4, 5],
        "Août - novembre": [8, 9, 10, 11],
        "Septembre": [9],
        "Juin - août": [6, 7, 8],
        "Août - septembre": [8, 9],
        "Mars - mai, septembre - novembre": [3, 4, 5, 9, 10, 11],
        "Avril - octobre": [4, 5, 6, 7, 8, 9, 10],
    }

    # 4. Dictionnaire de conversion de la taille textuelle vers les dimensions statistiques
    size_mapping = {
        "Minuscule": (1.0, 5.0, 1.5),
        "Petite": (5.0, 12.0, 3.0),
        "Petite/moyenne": (8.0, 18.0, 4.0),
        "Moyenne": (15.0, 25.0, 5.0),
        "Moyenne/grande": (20.0, 35.0, 7.0),
        "Grande": (30.0, 50.0, 10.0),
        "Très grande": (50.0, 80.0, 15.0),
        "Énorme": (80.0, 130.0, 25.0),
        "Énorme (aileron)": (100.0, 160.0, 30.0),
        "Fine": (10.0, 40.0, 10.0),
    }

    # 5. Parcourir et fusionner/ajouter les poissons AC et Fantasy
    all_to_process = ac_fishes + fantasy_fishes
    
    def determine_time_restriction(pref_time):
        if not pref_time:
            return None
        pref_time = pref_time.strip()
        if pref_time.lower() in ["toute la journée", "24h/24"]:
            return None
        
        match = re.search(r'^(\d+)\s*h', pref_time, re.IGNORECASE)
        if match:
            start_hour = int(match.group(1))
            if start_hour < 12:
                return "before_22h"
            else:
                return "after_22h"
        return None

    for nom, rareté_cible, lieu, prix, période, heures, restriction, taille_str, anecdote in all_to_process:
        # Chercher s'il existe déjà dans l'une des raretés du game_data
        existing_fish = None
        existing_rarity = None
        
        for r_key, f_list in data["fish_data"].items():
            for f in f_list:
                if f["name"].lower() == nom.lower():
                    existing_fish = f
                    existing_rarity = r_key
                    break
            if existing_fish:
                break
                
        # Conversion mois
        month_list = months_mapping.get(période, list(range(1, 13)))
        
        # Calcul dynamique de time_restriction
        dyn_restriction = determine_time_restriction(heures)
        
        if existing_fish:
            # Poisson existant : on ne modifie ABSOLUMENT RIEN pour respecter vos saisies manuelles !
            print(f"Préservation de {existing_fish['name']} existant dans '{existing_rarity}' (aucune modification)")
        else:
            # Poisson inexistant : le créer de toutes pièces
            print(f"Création de {nom} dans '{rareté_cible}'")
            s_min, s_mean, s_sigma = size_mapping.get(taille_str, (10.0, 30.0, 8.0))
            
            new_fish = {
                "name": nom,
                "size_min": s_min,
                "size_mean": s_mean,
                "size_sigma": s_sigma,
                "force_pristine": None,
                "force_state": None,
                "descriptions": {},
                "price": prix,
                "location": lieu,
                "preferred_time": heures,
                "preferred_season": période,
                "months": month_list,
                "fun_fact": anecdote,
                "time_restriction": dyn_restriction
            }
            
            # Appliquer les descriptions spécifiques ou génériques en français
            if nom.lower() == "têtard" or nom.lower() == "grenouille":
                apply_naruto_descriptions(new_fish, nom)
            elif nom == "Piranhator":
                new_fish["descriptions"] = {
                    "badly damaged": ["🔴 Piranhator désactivé, rouillé par la rivière et court-circuité."],
                    "damaged": ["🟠 Ses circuits cybernétiques sont endommagés, des étincelles jaillissent."],
                    "worn": ["🟡 Un cyborg piranha. Sa nageoire mécanique grince légèrement."],
                    "good": ["🟢 Un Piranhator en parfait état mécanique de fonctionnement."],
                    "pristine": ["✨ Un Piranhator flamboyant ! Ses implants chromés et ses yeux lasers brillent intensément ! 🤖🔥"]
                }
            elif nom == "Xénomorphe des Abysses":
                new_fish["descriptions"] = {
                    "badly damaged": ["🔴 Un reste informe de xénomorphe broyé par la pression abyssale."],
                    "damaged": ["🟠 La bête est blessée, son sang acide vert fluorescent corrode le filet."],
                    "worn": ["🟡 Il glisse silencieusement dans le noir, un prédateur parfait."],
                    "good": ["🟢 Une créature d'un noir d'encre dotée d'une double mâchoire interne effrayante."],
                    "pristine": ["✨ Un Xénomorphe des Abysses absolument terrifiant d'éclat ! L'acide bout dans ses veines ! 👽☠️"]
                }
            elif nom == "Le Crabe Dictateur":
                new_fish["descriptions"] = {
                    "badly damaged": ["🔴 Le tyran est tombé ! Ses pinces brisées ne font plus peur à personne."],
                    "damaged": ["🟠 Son armure impériale rouge est fissurée, mais sa fureur autoritaire reste entière."],
                    "worn": ["🟡 Un crabe en uniforme doté d'un petit chapeau d'amiral rigide."],
                    "good": ["🟢 Il claque fièrement de sa pince géante, ordonnant votre reddition immédiate."],
                    "pristine": ["👑 Le Crabe Dictateur brandissant sa pince en signe de domination absolue sur tout l'étang !"]
                }
            else:
                # Générique
                new_fish["descriptions"] = {
                    "badly damaged": [f"🔴 Un(e) {nom.lower()} en piteux état, très abîmé par le combat."],
                    "damaged": [f"🟠 Un(e) {nom.lower()} blessé(e) avec des marques d'écailles manquantes."],
                    "worn": [f"🟡 Un(e) {nom.lower()} ordinaire, un peu fatigué par le courant."],
                    "good": [f"🟢 Un(e) magnifique {nom.lower()} vigoureux et très frais de pêche."],
                    "pristine": [f"✨ Un spécimen de {nom.lower()} absolument parfait, resplendissant sous le soleil !"]
                }
                
            data["fish_data"][rareté_cible].append(new_fish)

    # 5.5 Attribuer des IDs séquentiels (Pokédex style) de 1 à N
    # Trier par niveau de rareté et par ordre alphabétique pour être déterministe
    rarity_order = ["common", "uncommon", "rare", "veryrare", "epic", "legendary", "mythical", "divin"]
    current_id = 1
    for r_key in rarity_order:
        if r_key in data["fish_data"]:
            data["fish_data"][r_key].sort(key=lambda x: x["name"].lower())
            for fish in data["fish_data"][r_key]:
                fish["id"] = current_id
                current_id += 1

    # 5.6 Assurer une structure 15-clés uniforme et ordonnée pour TOUS les poissons
    keys_order = [
        "name", "size_min", "size_mean", "size_sigma",
        "force_pristine", "force_state", "descriptions", "id",
        "price", "location", "preferred_time", "preferred_season",
        "months", "fun_fact", "time_restriction"
    ]
    for r_key, fishes in data["fish_data"].items():
        updated_fishes = []
        for fish in fishes:
            # Calcul dynamique de time_restriction si preferred_time est présent et restriction est None
            if fish.get("preferred_time") and not fish.get("time_restriction"):
                fish["time_restriction"] = determine_time_restriction(fish["preferred_time"])
            
            ordered_fish = {}
            for k in keys_order:
                ordered_fish[k] = fish.get(k, None)
            updated_fishes.append(ordered_fish)
        data["fish_data"][r_key] = updated_fishes

    # 6. Sauvegarder dans les fichiers de destination
    for p in [json_path, src_json_path]:
        os.makedirs(os.path.dirname(p), exist_ok=True)
        with open(p, 'w', encoding='utf-8') as f:
            json.dump(data, f, indent=4, ensure_ascii=False)
            
    print("Base de données JSON enrichie avec succès avec IDs dans les deux destinations !")

def apply_naruto_descriptions(fish_obj, name):
    if name.lower() == "têtard":
        fish_obj["descriptions"] = {
            "badly damaged": ["🔴 Un pauvre petit têtard tout égratigné. Jiraya n'aurait pas été fier."],
            "damaged": ["🟠 Un têtard abîmé. Son chakra semble complètement à plat."],
            "worn": ["🟡 Un têtard ordinaire. Il frétille doucement du bout de la queue."],
            "good": ["🟢 Un têtard vigoureux ! Il s'entraîne déjà à bondir de nénuphar en nénuphar."],
            "pristine": ["✨ Un têtard d'exception ! On dirait la réincarnation d'un têtard ninja du Mont Myoboku prêt à invoquer Gamabunta ! 🌀🐸"]
        }
    elif name.lower() == "grenouille":
        fish_obj["descriptions"] = {
            "badly damaged": ["🔴 Une grenouille mal en point, son chakra senjutsu semble totalement vidé."],
            "damaged": ["🟠 Une grenouille froissée, elle a dû rater son atterrissage depuis le mont Myoboku."],
            "worn": ["🟡 Une grenouille de rivière tout à fait classique."],
            "good": ["🟢 Une grenouille vigoureuse qui coasse fièrement. Jiraya approuverait son potentiel ! 🐸"],
            "pristine": ["✨ Une grenouille ninja légendaire du mont Myoboku dotée d'une puissance incroyable et prête pour le mode ermite ! 🐸🔥🌀"]
        }

if __name__ == "__main__":
    run_enrichment()
