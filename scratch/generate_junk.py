# -*- coding: utf-8 -*-
import json
import os

JUNK_FR_PATH = "data/junk_data.json"
JUNK_EN_PATH = "data/junk_data_en.json"

NOTORIETY_VALUES = {
    "common": (10, 20),
    "uncommon": (20, 40),
    "rare": (40, 80),
    "veryrare": (60, 120),
    "epic": (100, 200),
    "legendary": (200, 400),
    "mythical": (350, 700),
    "divin": (500, 1000)
}

# The 100 new items
new_items_data = [
    # --- COMMON (18 items) ---
    {
        "name_fr": "Gobelet en carton de marque",
        "name_en": "Branded paper cup",
        "rarity": "common",
        "bin": "jaune",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce gobelet à usage unique provient de la restauration rapide.",
        "degradation_fr": "son enveloppe en carton se désintègre vite mais son film plastique intérieur en polyéthylène persiste des décennies.",
        "impact_fr": "les animaux marins avalent les microparticules de plastique qui bloquent leur estomac.",
        "ecology_fr": "il libère des phtalates toxiques dans la chaîne trophique.",
        "history_en": "this single-use cup originates from fast food chains.",
        "degradation_en": "its paper shell disintegrates quickly but its inner polyethylene liner persists for decades.",
        "impact_en": "marine animals swallow the plastic microparticles which block their stomach.",
        "ecology_en": "it releases toxic phthalates into the trophic chain."
    },
    {
        "name_fr": "Pochette plastique transparente",
        "name_en": "Clear plastic sleeve",
        "rarity": "common",
        "bin": "jaune",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette pochette transparente sert à protéger les documents administratifs.",
        "degradation_fr": "le polypropylène dont elle est faite se fragmente sous l'effet du rayonnement ultraviolet marin.",
        "impact_fr": "les petits poissons la confondent avec du plancton ou des larves transparentes.",
        "ecology_fr": "elle engendre une pollution par microplastiques quasi indestructible à court terme.",
        "history_en": "this clear sleeve is used to protect administrative documents.",
        "degradation_en": "the polypropylene it is made of fragments under the effect of marine ultraviolet radiation.",
        "impact_en": "small fish mistake it for plankton or transparent larvae.",
        "ecology_en": "it creates a microplastic pollution that is almost indestructible in the short term."
    },
    {
        "name_fr": "Ticket de caisse thermique",
        "name_en": "Thermal cash receipt",
        "rarity": "common",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce ticket en papier thermique est délivré lors de chaque transaction commerciale.",
        "degradation_fr": "son papier se décompose en quelques mois mais libère immédiatement ses substances chimiques.",
        "impact_fr": "les composés chimiques qu'il contient pénètrent la peau des amphibiens.",
        "ecology_fr": "il contient du Bisphénol A (BPA) qui agit comme un puissant perturbateur endocrinien sur la faune.",
        "history_en": "this thermal paper receipt is issued during commercial transactions.",
        "degradation_en": "its paper decomposes in a few months but immediately releases its chemical substances.",
        "impact_en": "the chemical compounds it contains penetrate the skin of amphibians.",
        "ecology_en": "it contains Bisphenol A (BPA) which acts as a powerful endocrine disruptor on wildlife."
    },
    {
        "name_fr": "Pellicule de paquet de cigarettes",
        "name_en": "Cigarette pack plastic film",
        "rarity": "common",
        "bin": "jaune",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce film protecteur transparent enveloppe les paquets de tabac neufs.",
        "degradation_fr": "cette fine couche de plastique se fragmente rapidement en milliers de confettis invisibles.",
        "impact_fr": "les organismes filtreurs comme les moules et huîtres ingèrent ces fragments.",
        "ecology_fr": "elle accumule d'autres polluants organiques persistants présents dans l'eau.",
        "history_en": "this clear protective film wraps new tobacco packs.",
        "degradation_en": "this thin layer of plastic quickly fragments into thousands of invisible pieces.",
        "impact_en": "filtering organisms like mussels and oysters ingest these fragments.",
        "ecology_en": "it accumulates other persistent organic pollutants present in the water."
    },
    {
        "name_fr": "Emballage de chewing-gum",
        "name_en": "Chewing gum wrapper",
        "rarity": "common",
        "bin": "jaune",
        "gender": "M", "plural": False, "vowel_fr": True, "vowel_en": False,
        "history_fr": "cet emballage métallisé ou plastifié protège les tablettes de gomme à mâcher.",
        "degradation_fr": "l'assemblage de plastique et d'aluminium prend plus de cent ans à s'éroder.",
        "impact_fr": "les oiseaux marins ramassent ce déchet brillant pour nourrir leurs poussins.",
        "ecology_fr": "l'aluminium se corrode doucement tandis que le film plastique persiste indéfiniment.",
        "history_en": "this metallized or plasticized wrapper protects chewing gum tablets.",
        "degradation_en": "the assembly of plastic and aluminum takes over a hundred years to erode.",
        "impact_en": "seabirds pick up this shiny waste to feed their chicks.",
        "ecology_en": "the aluminum slowly corrodes while the plastic film persists indefinitely."
    },
    {
        "name_fr": "Capsule de bouteille en plastique",
        "name_en": "Plastic bottle cap",
        "rarity": "common",
        "bin": "jaune",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette capsule en plastique rigide scelle les bouteilles de boissons.",
        "degradation_fr": "composée de PEHD ou PP, elle résiste aux pires tempêtes sans se décomposer pendant des siècles.",
        "impact_fr": "les albatros adultes les confondent avec des calamars et s'étouffent avec.",
        "ecology_fr": "elle représente l'un des déchets les plus collectés lors des nettoyages de plages.",
        "history_en": "this rigid plastic cap seals beverage bottles.",
        "degradation_en": "made of HDPE or PP, it resists the worst storms without decomposing for centuries.",
        "impact_en": "adult albatrosses mistake them for squid and choke on them.",
        "ecology_en": "it represents one of the most collected waste items during beach cleanups."
    },
    {
        "name_fr": "Morceau de filet de tennis",
        "name_en": "Piece of tennis net",
        "rarity": "common",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce cordage provient d'un filet de tennis usé jeté à la poubelle.",
        "degradation_fr": "les mailles en nylon tressé résistent à l'usure mécanique et mettent 600 ans à s'effriter.",
        "impact_fr": "les petits poissons et crustacés s'emmêlent dans ses mailles serrées.",
        "ecology_fr": "il agit comme un mini filet fantôme piégeant la microfaune aquatique.",
        "history_en": "this rope comes from a worn tennis net thrown into the garbage.",
        "degradation_en": "the braided nylon meshes resist mechanical wear and take 600 years to crumble.",
        "impact_en": "small fish and shellfish get entangled in its tight mesh.",
        "ecology_en": "it acts as a mini ghost net trapping aquatic microfauna."
    },
    {
        "name_fr": "Opercule de pot de confiture",
        "name_en": "Jam jar lid",
        "rarity": "common",
        "bin": "jaune",
        "gender": "M", "plural": False, "vowel_fr": True, "vowel_en": False,
        "history_fr": "cet opercule métallique à vis assure l'étanchéité des bocaux en verre.",
        "degradation_fr": "le fer blanc se corrode au contact du sel en quelques années, laissant de la rouille.",
        "impact_fr": "les bords coupants de la capsule rouillée peuvent blesser les animaux fouisseurs.",
        "ecology_fr": "le joint en plastique polymère situé à l'intérieur se détache et pollue l'eau.",
        "history_en": "this metal screw lid seals glass jars.",
        "degradation_en": "the tinplate corrodes in contact with salt in a few years, leaving rust.",
        "impact_en": "the sharp edges of the rusty cap can injure burrowing animals.",
        "ecology_en": "the polymer plastic seal located inside detaches and pollutes the water."
    },
    {
        "name_fr": "Emballage de brosse à dents",
        "name_en": "Toothbrush blister pack",
        "rarity": "common",
        "bin": "jaune",
        "gender": "M", "plural": False, "vowel_fr": True, "vowel_en": False,
        "history_fr": "cet emballage mixte carton-plastique protège la brosse avant son premier usage.",
        "degradation_fr": "le carton se dissout rapidement tandis que la coque en PET reste intacte pendant 400 ans.",
        "impact_fr": "les petits animaux peuvent se retrouver piégés dans la coque plastique incurvée.",
        "ecology_fr": "la séparation difficile des deux matériaux complique le processus de tri sélectif.",
        "history_en": "this mixed cardboard-plastic packaging protects the brush before its first use.",
        "degradation_en": "the cardboard quickly dissolves while the PET shell remains intact for 400 years.",
        "impact_en": "small animals can get trapped inside the curved plastic shell.",
        "ecology_en": "the difficult separation of the two materials complicates the selective sorting process."
    },
    {
        "name_fr": "Filet à provisions en plastique",
        "name_en": "Plastic mesh produce bag",
        "rarity": "common",
        "bin": "jaune",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce filet léger sert à emballer les oignons, ails ou citrons au supermarché.",
        "degradation_fr": "ses fils de plastique extrêmement fins se rompent mais ne se dégradent pas chimiquement.",
        "impact_fr": "les oiseaux de mer s'entravent les pattes et le bec dans ses mailles élastiques.",
        "ecology_fr": "il contribue à la strangulation lente de nombreux animaux marins.",
        "history_en": "this lightweight mesh bag is used to pack onions, garlic or lemons at the supermarket.",
        "degradation_en": "its extremely thin plastic threads break but do not degrade chemically.",
        "impact_en": "seabirds get their legs and beaks entangled in its elastic meshes.",
        "ecology_en": "it contributes to the slow strangulation of many marine animals."
    },
    {
        "name_fr": "Pot de yaourt vide",
        "name_en": "Empty yogurt pot",
        "rarity": "common",
        "bin": "jaune",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "ce pot en polystyrène contenait du yaourt avant d'être jeté.",
        "degradation_fr": "le polystyrène devient cassant et se brise en des milliers de micro-morceaux blancs.",
        "impact_fr": "les poissons confondent ces éclats blancs avec des œufs de poisson.",
        "ecology_fr": "les additifs chimiques du polystyrène s'accumulent dans les tissus gras de la faune.",
        "history_en": "this polystyrene pot contained yogurt before being thrown away.",
        "degradation_en": "the polystyrene becomes brittle and breaks into thousands of tiny white pieces.",
        "impact_en": "fish mistake these white shards for fish eggs.",
        "ecology_en": "the chemical additives of polystyrene accumulate in the fatty tissues of wildlife."
    },
    {
        "name_fr": "Bouteille de shampoing vide",
        "name_en": "Empty shampoo bottle",
        "rarity": "common",
        "bin": "jaune",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "cette bouteille en plastique opaque contenait du shampoing ou du gel douche.",
        "degradation_fr": "le polyéthylène haute densité (PEHD) met jusqu'à 500 ans à s'éroder complètement.",
        "impact_fr": "elle flotte à la dérive et sert de support à des espèces invasives qui colonisent d'autres milieux.",
        "ecology_fr": "les résidus de savon à l'intérieur polluent chimiquement les nappes d'eau locales.",
        "history_en": "this opaque plastic bottle contained shampoo or shower gel.",
        "degradation_en": "the high-density polyethylene (HDPE) takes up to 500 years to fully erode.",
        "impact_en": "it drifts along the surface and serves as a raft for invasive species to colonize new environments.",
        "ecology_en": "the soap residues inside chemically pollute the local water bodies."
    },
    {
        "name_fr": "Tube de colle vide",
        "name_en": "Empty glue tube",
        "rarity": "common",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "ce petit tube en plastique ou aluminium contenait de la colle forte.",
        "degradation_fr": "le plastique durci et les solvants résiduels ralentissent la biodégradation naturelle.",
        "impact_fr": "les solvants toxiques résiduels empoisonnent directement les micro-organismes aquatiques.",
        "ecology_fr": "c'est un déchet chimique complexe qui doit impérativement rejoindre la décharge.",
        "history_en": "this small plastic or aluminum tube contained superglue.",
        "degradation_en": "the hardened plastic and residual solvents slow down natural biodegradation.",
        "impact_en": "residual toxic solvents directly poison aquatic micro-organisms.",
        "ecology_en": "it is a complex chemical waste that must absolutely go to the landfill."
    },
    {
        "name_fr": "Lacet de chaussure synthétique",
        "name_en": "Synthetic shoelace",
        "rarity": "common",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce lacet en polyester s'est détaché d'une vieille chaussure jetée.",
        "degradation_fr": "le polyester synthétique met plusieurs siècles à se décomposer sous l'action de l'eau.",
        "impact_fr": "les oiseaux d'eau l'utilisent pour construire leur nid, étranglant parfois leurs poussins.",
        "ecology_fr": "les colorants synthétiques se dissolvent lentement dans l'écosystème aquatique.",
        "history_en": "this polyester shoelace detached from a discarded shoe.",
        "degradation_en": "the synthetic polyester takes several centuries to decompose under the action of water.",
        "impact_en": "waterbirds use it to build their nests, sometimes strangling their chicks.",
        "ecology_en": "synthetic dyes slowly dissolve into the aquatic ecosystem."
    },
    {
        "name_fr": "Morceau de polystyrène expansé",
        "name_en": "Piece of expanded polystyrene",
        "rarity": "common",
        "bin": "jaune",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce bloc de mousse blanche servait à caler un appareil électroménager.",
        "degradation_fr": "il s'émiette extrêmement vite en petites billes blanches sous l'action mécanique des vagues.",
        "impact_fr": "les poissons et oiseaux mangent ces billes légères, provoquant leur mort par famine.",
        "ecology_fr": "composé à 98% d'air, sa structure plastique est pourtant éternelle à l'échelle humaine.",
        "history_en": "this white foam block was used to cushion a household appliance.",
        "degradation_en": "it crumbles extremely fast into small white beads under the mechanical action of waves.",
        "impact_en": "fish and birds eat these lightweight beads, causing death by starvation.",
        "ecology_en": "composed of 98% air, its plastic structure is nevertheless eternal on a human scale."
    },
    {
        "name_fr": "Couvercle de boîte de conserve",
        "name_en": "Tin can lid",
        "rarity": "common",
        "bin": "jaune",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce couvercle en acier a été détaché d'une boîte de conserve de légumes.",
        "degradation_fr": "le fer blanc s'oxyde rapidement en eau douce ou salée pour finir par se dissoudre.",
        "impact_fr": "les bords tranchants de la tôle coupent les pattes des animaux côtiers.",
        "ecology_fr": "bien que recyclable à l'infini, sa forme tranchante en fait un danger immédiat.",
        "history_en": "this steel lid was detached from a vegetable tin can.",
        "degradation_en": "the tinplate oxidizes quickly in fresh or salt water to eventually dissolve.",
        "impact_en": "the sharp edges of the sheet metal cut the paws and legs of coastal animals.",
        "ecology_en": "although infinitely recyclable, its sharp shape makes it an immediate danger."
    },
    {
        "name_fr": "Blister de médicaments vide",
        "name_en": "Empty pill blister pack",
        "rarity": "common",
        "bin": "jaune",
        "gender": "M", "plural": False, "vowel_fr": True, "vowel_en": True,
        "history_fr": "cet emballage plastique et aluminium contenait des gélules médicamenteuses.",
        "degradation_fr": "le plastique PVC et l'aluminium collés ensemble empêchent une dégradation rapide.",
        "impact_fr": "les reflets brillants attirent les prédateurs marins qui s'empoisonnent.",
        "ecology_fr": "ce déchet composite est difficile à recycler et finit souvent incinéré.",
        "history_en": "this plastic and aluminum packaging contained medical capsules.",
        "degradation_en": "the PVC plastic and aluminum glued together prevent rapid degradation.",
        "impact_en": "the shiny reflections attract marine predators which poison themselves.",
        "ecology_en": "this composite waste is difficult to recycle and often ends up incinerated."
    },
    {
        "name_fr": "Paille en papier détrempée",
        "name_en": "Soggy paper straw",
        "rarity": "common",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette paille en papier a été utilisée pour boire un soda avant d'échouer.",
        "degradation_fr": "le papier se délite en quelques jours mais les colles chimiques utilisées persistent.",
        "impact_fr": "les animaux risquent d'ingérer la pâte de papier saturée de produits chimiques.",
        "ecology_fr": "sa fabrication requiert des liants synthétiques qui polluent l'eau lors de sa dissolution.",
        "history_en": "this paper straw was used to drink a soda before drifting away.",
        "degradation_en": "the paper disintegrates in a few days but the chemical glues used persist.",
        "impact_en": "animals risk ingesting the paper pulp saturated with chemical products.",
        "ecology_en": "its manufacturing requires synthetic binders that pollute the water during its dissolution."
    },

    # --- UNCOMMON (20 items) ---
    {
        "name_fr": "Brosse à cheveux cassée",
        "name_en": "Broken hairbrush",
        "rarity": "uncommon",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette brosse à cheveux en plastique dur s'est brisée après un long usage.",
        "degradation_fr": "le plastique moulé et les picots en nylon mettent plus de 500 ans à disparaître.",
        "impact_fr": "les picots pointus s'accrochent aux branchies des poissons.",
        "ecology_fr": "elle constitue un déchet composite complexe qui ne peut pas être recyclé en poubelle jaune.",
        "history_en": "this hard plastic hairbrush broke after a long period of use.",
        "degradation_en": "the molded plastic and nylon bristles take over 500 years to disappear.",
        "impact_en": "the sharp bristles hook onto fish gills.",
        "ecology_en": "it constitutes a complex composite waste that cannot be recycled in the yellow bin."
    },
    {
        "name_fr": "Jouet de plage (seau)",
        "name_en": "Beach toy bucket",
        "rarity": "uncommon",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce seau en plastique de couleur vive a été oublié par un enfant sur le sable.",
        "degradation_fr": "exposé au sel et au vent, il se fissure puis éclate en morceaux durs.",
        "impact_fr": "les animaux fouisseurs se coupent sur les éclats de plastique tranchants.",
        "ecology_fr": "les jouets en plastique ne font pas partie des emballages et doivent aller en décharge.",
        "history_en": "this brightly colored plastic bucket was forgotten by a child on the sand.",
        "degradation_en": "exposed to salt and wind, it cracks and then breaks into hard pieces.",
        "impact_en": "burrowing animals cut themselves on the sharp plastic shards.",
        "ecology_en": "plastic toys are not classified as packaging and must go to the landfill."
    },
    {
        "name_fr": "Lunettes de soleil sans verres",
        "name_en": "Sunglasses frame",
        "rarity": "uncommon",
        "bin": "decharge",
        "gender": "F", "plural": True, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette monture de lunettes de soleil en plastique a perdu ses verres protecteurs.",
        "degradation_fr": "les branches en acétate et les charnières métalliques mettent des siècles à s'éroder.",
        "impact_fr": "la forme en boucle fermée de la monture peut piéger de petits oiseaux ou poissons.",
        "ecology_fr": "les additifs chimiques de l'acétate s'infiltrent lentement dans le sable et l'eau.",
        "history_en": "this plastic sunglasses frame has lost its protective lenses.",
        "degradation_en": "the acetate temples and metal hinges take centuries to erode.",
        "impact_en": "the closed loop shape of the frame can trap small birds or fish.",
        "ecology_en": "the chemical additives of the acetate slowly seep into the sand and water."
    },
    {
        "name_fr": "Coque de smartphone en silicone",
        "name_en": "Silicone smartphone case",
        "rarity": "uncommon",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette protection souple pour téléphone a été jetée après changement de modèle.",
        "degradation_fr": "le silicone est un polymère synthétique hautement résistant qui ne se décompose pas.",
        "impact_fr": "les tortues de mer peuvent confondre sa texture souple avec une proie comestible.",
        "ecology_fr": "non recyclable dans les filières classiques, elle finit enfouie ou incinérée.",
        "history_en": "this flexible phone protection was discarded after a model change.",
        "degradation_en": "silicone is a highly resistant synthetic polymer that does not decompose.",
        "impact_en": "sea turtles can mistake its flexible texture for edible prey.",
        "ecology_en": "non-recyclable in classic sorting facilities, it ends up buried or incinerated."
    },
    {
        "name_fr": "Clé USB défectueuse",
        "name_en": "Broken USB flash drive",
        "rarity": "uncommon",
        "bin": "gris",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette petite clé de stockage de données ne fonctionne plus.",
        "degradation_fr": "les composants électroniques internes s'oxydent lentement, libérant des métaux lourds.",
        "impact_fr": "les prédateurs aquatiques peuvent avaler cet objet brillant métallique.",
        "ecology_fr": "elle contient des puces en silicium et du cuivre qui doivent être recyclés en bac DEEE.",
        "history_en": "this small data storage drive no longer functions.",
        "degradation_en": "the internal electronic components slowly oxidize, releasing heavy metals.",
        "impact_en": "aquatic predators can swallow this shiny metallic object.",
        "ecology_en": "it contains silicon chips and copper that must be recycled in WEEE bins."
    },
    {
        "name_fr": "Ampoule LED grillée",
        "name_en": "Burnt-out LED bulb",
        "rarity": "uncommon",
        "bin": "gris",
        "gender": "F", "plural": False, "vowel_fr": True, "vowel_en": False,
        "history_fr": "cette ampoule LED a éclairé une maison avant de rendre l'âme.",
        "degradation_fr": "le plastique diffuseur résiste tandis que le culot métallique s'oxyde lentement.",
        "impact_fr": "les composants internes contiennent de l'arsenic et du plomb toxiques pour la faune.",
        "ecology_fr": "le recyclage des LED permet de récupérer des terres rares indispensables à l'industrie.",
        "history_en": "this LED bulb illuminated a home before burning out.",
        "degradation_en": "the plastic diffuser resists while the metallic base slowly oxidizes.",
        "impact_en": "the internal components contain arsenic and lead that are toxic to wildlife.",
        "ecology_en": "recycling LEDs allows the recovery of rare earths essential for the industry."
    },
    {
        "name_fr": "Écouteurs filaires emmêlés",
        "name_en": "Tangled wired earphones",
        "rarity": "uncommon",
        "bin": "gris",
        "gender": "M", "plural": True, "vowel_fr": True, "vowel_en": True,
        "history_fr": "ces écouteurs de musique se sont cassés après avoir été emmêlés.",
        "degradation_fr": "le gainage en plastique souple protège les fils de cuivre de l'érosion pendant des décennies.",
        "impact_fr": "les oiseaux aquatiques s'entravent les ailes dans les longs fils fins.",
        "ecology_fr": "ils contiennent du cuivre et des métaux précieux précieux pour le recyclage électronique.",
        "history_en": "these music earphones broke after being tangled up.",
        "degradation_en": "the flexible plastic sheathing protects the copper wires from erosion for decades.",
        "impact_en": "waterbirds get their wings entangled in the long thin wires.",
        "ecology_en": "they contain copper and precious metals valuable for electronic recycling."
    },
    {
        "name_fr": "Rallonge électrique dénudée",
        "name_en": "Stripped extension cord",
        "rarity": "uncommon",
        "bin": "gris",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "cette rallonge a été jetée après que sa gaine isolante se soit abîmée.",
        "degradation_fr": "sa gaine extérieure en PVC résiste tandis que les fils de cuivre intérieurs s'oxydent.",
        "impact_fr": "les poissons peuvent s'intoxiquer avec les plastifiants toxiques libérés par le PVC.",
        "ecology_fr": "les câbles électriques sont d'excellentes sources de cuivre recyclé pour l'industrie.",
        "history_en": "this extension cord was discarded after its insulating sheath was damaged.",
        "degradation_en": "its outer PVC sheath resists while the internal copper wires oxidize.",
        "impact_en": "fish can be poisoned by the toxic plasticizers released by the PVC.",
        "ecology_en": "electric cables are excellent sources of recycled copper for the industry."
    },
    {
        "name_fr": "Disquette 3.5 pouces",
        "name_en": "3.5-inch floppy disk",
        "rarity": "uncommon",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce support de stockage obsolète date des années 1990.",
        "degradation_fr": "sa coque en plastique rigide et son disque magnétique intérieur mettent 500 ans à se dégrader.",
        "impact_fr": "les fragments de plastique tranchants de la coque brisée blessent la faune marine.",
        "ecology_fr": "sa composition mêlant plastique dur, métal et bande magnétique la rend non recyclable.",
        "history_en": "this obsolete storage media dates back to the 1990s.",
        "degradation_en": "its rigid plastic shell and internal magnetic disk take 500 years to degrade.",
        "impact_en": "sharp plastic fragments from the broken shell injure marine life.",
        "ecology_en": "its composition mixing hard plastic, metal and magnetic tape makes it non-recyclable."
    },
    {
        "name_fr": "Boîtier de DVD cassé",
        "name_en": "Broken DVD case",
        "rarity": "uncommon",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce boîtier noir protégeait un film en DVD avant de se fendre.",
        "degradation_fr": "constitué de polypropylène rigide, sa vitesse de décomposition est estimée à 400 ans.",
        "impact_fr": "les éclats de plastique pointus suite à sa rupture peuvent perforer l'estomac des animaux.",
        "ecology_fr": "le polypropylène des boîtiers n'étant pas un emballage, il doit rejoindre les ordures ménagères.",
        "history_en": "this black case protected a DVD film before cracking.",
        "degradation_en": "made of rigid polypropylene, its decomposition rate is estimated at 400 years.",
        "impact_en": "sharp plastic shards following its rupture can perforate the stomach of animals.",
        "ecology_en": "the polypropylene of cases not being packaging, it must go to the household waste."
    },
    {
        "name_fr": "Livre de poche délavé",
        "name_en": "Faded paperback book",
        "rarity": "uncommon",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce roman de poche a été perdu au bord de l'eau par un lecteur distrait.",
        "degradation_fr": "les pages se détrempent et se transforment en pâte à papier en quelques semaines.",
        "impact_fr": "les encres chimiques industrielles se dispersent directement dans l'écosystème aquatique.",
        "ecology_fr": "mouillé et couvert de limon, le papier perd toute sa recyclabilité et doit aller en décharge.",
        "history_en": "this paperback novel was lost by the water's edge by a distracted reader.",
        "degradation_en": "the pages soak and turn into paper pulp in a few weeks.",
        "impact_en": "industrial chemical inks disperse directly into the aquatic ecosystem.",
        "ecology_en": "wet and covered with silt, the paper loses all its recyclability and must go to the landfill."
    },
    {
        "name_fr": "Cahier à spirales rouillées",
        "name_en": "Notebook with rusty spirals",
        "rarity": "uncommon",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce cahier d'écolier a fini dans l'eau avec ses notes manuscrites.",
        "degradation_fr": "le papier se décompose vite mais la spirale en fer rouille lentement et subsiste.",
        "impact_fr": "la spirale métallique rouillée peut agir comme un piège mécanique pour la microfaune.",
        "ecology_fr": "les colles de reliure et les encres plastifiées polluent durablement le biotope.",
        "history_en": "this school notebook ended up in the water with its handwritten notes.",
        "degradation_en": "the paper decomposes quickly but the iron spiral rusts slowly and remains.",
        "impact_en": "the rusty metal spiral can act as a mechanical trap for microfauna.",
        "ecology_en": "binding glues and plasticized inks pollute the biotope in the long term."
    },
    {
        "name_fr": "Bocal en verre sans couvercle",
        "name_en": "Glass jar without lid",
        "rarity": "uncommon",
        "bin": "vert",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce pot en verre épais contenait autrefois des conserves de légumes.",
        "degradation_fr": "le verre ne se décompose jamais mais s'érode lentement sous l'effet des vagues.",
        "impact_fr": "il peut abriter de petits organismes mais ses cassures coupent les poissons.",
        "ecology_fr": "le verre est recyclable à 100% à l'infini et doit rejoindre le bac à verre vert.",
        "history_en": "this thick glass jar once contained vegetable preserves.",
        "degradation_en": "glass never decomposes but slowly erodes under the effect of waves.",
        "impact_en": "it can shelter small organisms but its breaks cut fish.",
        "ecology_en": "glass is 100% infinitely recyclable and must go to the green glass bin."
    },
    {
        "name_fr": "Flacon de parfum vide",
        "name_en": "Empty perfume bottle",
        "rarity": "uncommon",
        "bin": "vert",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "ce flacon de parfum en verre de luxe a été jeté après usage.",
        "degradation_fr": "le verre résiste au temps tandis que le vaporisateur en plastique s'abîme doucement.",
        "impact_fr": "les résidus de parfum concentré peuvent être toxiques pour les micro-organismes.",
        "ecology_fr": "il convient de retirer la pompe en plastique avant de jeter le verre au bac vert.",
        "history_en": "this luxury glass perfume bottle was discarded after use.",
        "degradation_en": "the glass resists time while the plastic spray pump slowly decays.",
        "impact_en": "concentrated perfume residues can be toxic to micro-organisms.",
        "ecology_en": "the plastic pump should be removed before throwing the glass into the green bin."
    },
    {
        "name_fr": "Canette de soda en acier",
        "name_en": "Steel soda can",
        "rarity": "uncommon",
        "bin": "jaune",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette canette métallique de boisson gazeuse a été jetée dans la nature.",
        "degradation_fr": "l'acier se corrode et se désagrège en oxyde de fer en une dizaine d'années.",
        "impact_fr": "les rebords de l'ouverture peuvent blesser la bouche des poissons curieux.",
        "ecology_fr": "le recyclage de l'acier permet d'économiser jusqu'à 75% d'énergie par rapport au fer vierge.",
        "history_en": "this metal carbonated drink can was littered in nature.",
        "degradation_en": "the steel corrodes and disintegrates into iron oxide in about ten years.",
        "impact_en": "the edges of the opening can injure the mouth of curious fish.",
        "ecology_en": "recycling steel saves up to 75% of energy compared to virgin iron."
    },
    {
        "name_fr": "Brique de lait en carton",
        "name_en": "Cardboard milk carton",
        "rarity": "uncommon",
        "bin": "jaune",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cet emballage multicouche contenait du lait pasteurisé.",
        "degradation_fr": "le carton se dissout mais les couches de polyéthylène et d'aluminium durent 100 ans.",
        "impact_fr": "le plastique restant étouffe les coraux et empêche la lumière d'atteindre les plantes.",
        "ecology_fr": "ce déchet complexe nécessite un traitement industriel spécifique en poubelle jaune.",
        "history_en": "this multilayer packaging contained pasteurized milk.",
        "degradation_en": "the cardboard dissolves but the polyethylene and aluminum layers last 100 years.",
        "impact_en": "the remaining plastic smothers corals and prevents light from reaching plants.",
        "ecology_en": "this complex waste requires specific industrial treatment in the yellow bin."
    },
    {
        "name_fr": "Bouteille de lait en PEHD",
        "name_en": "HDPE milk bottle",
        "rarity": "uncommon",
        "bin": "jaune",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "cette bouteille blanche opaque servait à stocker du lait frais.",
        "degradation_fr": "sa structure moléculaire en polyéthylène haute densité met 500 ans à se briser.",
        "impact_fr": "les animaux marins peuvent s'y coincer la tête à la recherche de nourriture.",
        "ecology_fr": "le PEHD recyclé est utilisé pour fabriquer de nouveaux tuyaux ou des conteneurs.",
        "history_en": "this opaque white bottle was used to store fresh milk.",
        "degradation_en": "its high-density polyethylene molecular structure takes 500 years to break.",
        "impact_en": "marine animals can get their heads stuck inside searching for food.",
        "ecology_en": "recycled HDPE is used to manufacture new pipes or containers."
    },
    {
        "name_fr": "Bombe aérosol vide",
        "name_en": "Empty aerosol can",
        "rarity": "uncommon",
        "bin": "jaune",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "cet aérosol contenait du désodorisant ou de la laque pour cheveux.",
        "degradation_fr": "le boîtier en aluminium ou acier se corrode lentement en mer en 20 ans.",
        "impact_fr": "les restes de gaz propulseurs ou solvants polluent la nappe d'eau superficielle.",
        "ecology_fr": "vide, elle se recycle facilement avec les métaux dans la poubelle jaune.",
        "history_en": "this aerosol can contained air freshener or hairspray.",
        "degradation_en": "the aluminum or steel canister slowly corrodes in sea water in 20 years.",
        "impact_en": "remnants of propellant gases or solvents pollute the surface water layer.",
        "ecology_en": "when empty, it easily recycles with metals in the yellow bin."
    },
    {
        "name_fr": "Boîte de sardines en conserve",
        "name_en": "Sardine tin",
        "rarity": "uncommon",
        "bin": "jaune",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "cette boîte métallique scellée contenait des sardines à l'huile.",
        "degradation_fr": "l'acier étamé s'oxyde et disparaît complètement en milieu salin en quelques années.",
        "impact_fr": "le couvercle en métal découpé présente des arêtes extrêmement coupantes pour les poissons.",
        "ecology_fr": "elle contient des traces d'huile qui forment une fine pellicule grasse en surface.",
        "history_en": "this sealed metal can contained sardines in oil.",
        "degradation_en": "the tinplate steel oxidizes and completely disappears in saline environment in a few years.",
        "impact_en": "the cut metal lid presents extremely sharp edges for fish.",
        "ecology_en": "it contains traces of oil that form a thin greasy film on the surface."
    },
    {
        "name_fr": "Gourde en aluminium cabossée",
        "name_en": "Dented aluminum bottle",
        "rarity": "uncommon",
        "bin": "jaune",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "cette gourde réutilisable en métal a été égarée lors d'une randonnée.",
        "degradation_fr": "l'aluminium se ternit et s'oxyde lentement mais résiste des dizaines d'années.",
        "impact_fr": "le bouchon en plastique attaché peut être grignoté et avalé par les oiseaux.",
        "ecology_fr": "bien que conçue pour éviter les déchets, elle devient une pollution si elle est perdue.",
        "history_en": "this reusable metal flask was lost during a hike.",
        "degradation_en": "the aluminum tarnishes and slowly oxidizes but resists for decades.",
        "impact_en": "the attached plastic cap can be nibbled and swallowed by birds.",
        "ecology_en": "although designed to reduce waste, it becomes pollution if lost."
    },

    # --- RARE (20 items) ---
    {
        "name_fr": "Souris d'ordinateur sans fil",
        "name_en": "Wireless computer mouse",
        "rarity": "rare",
        "bin": "gris",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce périphérique informatique sans fil a cessé de fonctionner.",
        "degradation_fr": "le plastique ABS de la coque résiste tandis que les circuits imprimés s'altèrent.",
        "impact_fr": "la pile alcaline restée à l'intérieur fuit et déverse des acides hautement toxiques.",
        "ecology_fr": "le plastique et les métaux doivent être triés dans les réseaux DEEE appropriés.",
        "history_en": "this wireless computer peripheral ceased to function.",
        "degradation_en": "the ABS plastic of the shell resists while the circuit boards decay.",
        "impact_en": "the alkaline battery left inside leaks and pours highly toxic acids.",
        "ecology_en": "the plastic and metals must be sorted in the appropriate WEEE networks."
    },
    {
        "name_fr": "Chargeur de téléphone portable",
        "name_en": "Mobile phone charger",
        "rarity": "rare",
        "bin": "gris",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce bloc de charge secteur a été jeté après l'achat d'un nouveau téléphone.",
        "degradation_fr": "la coque en polycarbonate dure des siècles tandis que les broches métalliques rouillent.",
        "impact_fr": "le câble attaché peut s'enrouler autour des membres des oiseaux ou tortues.",
        "ecology_fr": "les composants internes contiennent du cuivre pur recyclable.",
        "history_en": "this mains charger block was discarded after buying a new phone.",
        "degradation_en": "the polycarbonate shell lasts for centuries while the metal pins rust.",
        "impact_en": "the attached cable can wrap around the limbs of birds or turtles.",
        "ecology_en": "the internal components contain recyclable pure copper."
    },
    {
        "name_fr": "Lecteur MP3 HS",
        "name_en": "Broken MP3 player",
        "rarity": "rare",
        "bin": "gris",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "cet appareil de musique portable des années 2000 est tombé en panne.",
        "degradation_fr": "l'écran à cristaux liquides et la batterie intégrée se détériorent lentement.",
        "impact_fr": "le lithium de la batterie peut s'enflammer au contact de l'eau en cas de fissure.",
        "ecology_fr": "les circuits contiennent de l'or et d'autres métaux rares précieux pour l'économie.",
        "history_en": "this portable music player from the 2000s broke down.",
        "degradation_en": "the liquid crystal display and the built-in battery slowly deteriorate.",
        "impact_en": "the lithium in the battery can ignite in contact with water if cracked.",
        "ecology_en": "the circuits contain gold and other rare metals precious to the economy."
    },
    {
        "name_fr": "Calculatrice scientifique",
        "name_en": "Scientific calculator",
        "rarity": "rare",
        "bin": "gris",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette calculatrice a servi à des générations d'étudiants avant d'échouer.",
        "degradation_fr": "son boîtier en plastique épais protège les circuits pendant plusieurs siècles.",
        "impact_fr": "les touches en caoutchouc souple peuvent être ingérées par les poissons de fond.",
        "ecology_fr": "les écrans et processeurs doivent rejoindre la filière de recyclage gris.",
        "history_en": "this calculator served generations of students before drifting away.",
        "degradation_en": "its thick plastic casing protects the circuits for several centuries.",
        "impact_en": "the soft rubber keys can be ingested by bottom-dwelling fish.",
        "ecology_en": "the screens and processors must join the gray recycling channel."
    },
    {
        "name_fr": "Console de jeux portable cassée",
        "name_en": "Broken handheld console",
        "rarity": "rare",
        "bin": "gris",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette console portable a diverti des joueurs avant d'être oubliée.",
        "degradation_fr": "les plastiques durs et l'écran brisé mettent 500 ans à se dégrader chimiquement.",
        "impact_fr": "les composants soudés au plomb polluent l'eau environnante de métaux toxiques.",
        "ecology_fr": "les batteries au lithium exigent un recyclage sécurisé dans le bac gris.",
        "history_en": "this handheld console entertained players before being forgotten.",
        "degradation_en": "the hard plastics and broken screen take 500 years to chemically degrade.",
        "impact_en": "the lead-soldered components pollute the surrounding water with toxic metals.",
        "ecology_en": "lithium batteries require secure recycling in the gray bin."
    },
    {
        "name_fr": "Disque dur externe rouillé",
        "name_en": "Rusty external hard drive",
        "rarity": "rare",
        "bin": "gris",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "cet appareil de sauvegarde de données a pris l'eau après une inondation.",
        "degradation_fr": "le boîtier en aluminium résiste mais les disques magnétiques internes s'oxydent.",
        "impact_fr": "le revêtement magnétique des disques se détache et contamine l'eau de cobalt.",
        "ecology_fr": "les aimants puissants en néodyme à l'intérieur contiennent des terres rares à recycler.",
        "history_en": "this data backup device was flooded after a heavy rain.",
        "degradation_en": "the aluminum casing resists but the internal magnetic disks oxidize.",
        "impact_en": "the magnetic coating of the disks detaches and contaminates the water with cobalt.",
        "ecology_en": "the powerful neodymium magnets inside contain rare earths to recycle."
    },
    {
        "name_fr": "Pèse-personne électronique",
        "name_en": "Electronic bathroom scale",
        "rarity": "rare",
        "bin": "gris",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "cet appareil de mesure du poids a été remplacé par un modèle connecté.",
        "degradation_fr": "le plateau en verre trempé résiste indéfiniment tandis que l'électronique s'oxyde.",
        "impact_fr": "les débris de verre trempé suite à une rupture coupent la faune benthique.",
        "ecology_fr": "sa structure mixte exige un démontage industriel pour séparer le verre des circuits.",
        "history_en": "this weight measuring device was replaced by a smart model.",
        "degradation_en": "the tempered glass platform resists indefinitely while the electronics oxidize.",
        "impact_en": "tempered glass shards following a break cut the benthic wildlife.",
        "ecology_en": "its mixed structure requires industrial dismantling to separate glass from circuits."
    },
    {
        "name_fr": "Grille-pain carbonisé",
        "name_en": "Charred toaster",
        "rarity": "rare",
        "bin": "gris",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce grille-pain a brûlé suite à un court-circuit électrique.",
        "degradation_fr": "les résistances internes en mica et nickel-chrome résistent très bien à la corrosion.",
        "impact_fr": "les miettes calcinées et la suie à l'intérieur altèrent la clarté de l'eau.",
        "ecology_fr": "sa coque métallique ou en plastique dur doit être triée dans la filière DEEE.",
        "history_en": "this toaster burned due to an electrical short circuit.",
        "degradation_en": "the internal mica and nichrome resistances resist corrosion very well.",
        "impact_en": "the charred crumbs and soot inside alter the clarity of the water.",
        "ecology_en": "its metal or hard plastic shell must be sorted in the WEEE channel."
    },
    {
        "name_fr": "Bouilloire électrique",
        "name_en": "Electric kettle",
        "rarity": "rare",
        "bin": "gris",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "cette bouilloire a chauffé de l'eau pour le thé avant de tomber en panne.",
        "degradation_fr": "la coque en plastique ou métal protège la résistance chauffante sous l'eau.",
        "impact_fr": "le tartre accumulé se détache tandis que les plastifiants polluent lentement.",
        "ecology_fr": "le socle électrique et le corps de l'appareil doivent être collectés ensemble dans le bac gris.",
        "history_en": "this kettle heated water for tea before breaking down.",
        "degradation_en": "the plastic or metal body protects the heating element under water.",
        "impact_en": "the accumulated scale detaches while the plasticizers slowly pollute.",
        "ecology_en": "the electrical base and the body of the appliance must be collected together in the gray bin."
    },
    {
        "name_fr": "Sèche-cheveux en panne",
        "name_en": "Broken hairdryer",
        "rarity": "rare",
        "bin": "gris",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cet appareil de coiffure a cessé de souffler de l'air chaud.",
        "degradation_fr": "son moteur électrique et son hélice en plastique mettent des siècles à s'éroder.",
        "impact_fr": "le long cordon d'alimentation représente un risque mortel de strangulation pour les poissons.",
        "ecology_fr": "le recyclage de son moteur permet de récupérer des bobinages de cuivre précieux.",
        "history_en": "this styling device stopped blowing hot air.",
        "degradation_en": "its electric motor and plastic fan take centuries to erode.",
        "impact_en": "the long power cord represents a mortal strangulation risk for fish.",
        "ecology_en": "recycling its motor allows the recovery of valuable copper windings."
    },
    {
        "name_fr": "Tire-bouchon en fonte",
        "name_en": "Cast iron corkscrew",
        "rarity": "rare",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce tire-bouchon lourd en fonte servait à ouvrir les bouteilles de vin.",
        "degradation_fr": "la fonte ferreuse s'oxyde lentement en créant une croûte de rouille protectrice.",
        "impact_fr": "sa mèche en spirale pointue constitue un danger de perforation pour les poissons.",
        "ecology_fr": "trop lourd pour le tri ménager jaune, il doit être déposé en déchetterie.",
        "history_en": "this heavy cast iron corkscrew was used to open wine bottles.",
        "degradation_en": "the ferrous cast iron slowly oxidizes, creating a protective rust crust.",
        "impact_en": "its sharp spiral worm constitutes a perforation hazard for fish.",
        "ecology_en": "too heavy for yellow household sorting, it must be deposited at the waste reception center."
    },
    {
        "name_fr": "Poêle antiadhésive rayée",
        "name_en": "Scratched non-stick pan",
        "rarity": "rare",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette poêle de cuisine a perdu ses propriétés antiadhésives à cause des rayures.",
        "degradation_fr": "l'aluminium se dégrade lentement tandis que le téflon extérieur s'écaille.",
        "impact_fr": "le téflon (PTFE) libère des microparticules de polymères fluorés indestructibles.",
        "ecology_fr": "les composés perfluorés (PFC) contaminent durablement les ressources en eau douce.",
        "history_en": "this kitchen pan lost its non-stick properties due to scratches.",
        "degradation_en": "the aluminum degrades slowly while the outer teflon peels off.",
        "impact_en": "the teflon (PTFE) releases microparticles of indestructible fluorated polymers.",
        "ecology_en": "perfluorinated compounds (PFCs) contaminate freshwater resources in the long term."
    },
    {
        "name_fr": "Plaque de cuisson en fonte",
        "name_en": "Cast iron griddle plate",
        "rarity": "rare",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette lourde plaque servait à griller les aliments sur le feu.",
        "degradation_fr": "la fonte de fer épaisse s'érode très lentement sur plusieurs décennies en mer.",
        "impact_fr": "son poids élevé écrase les herbiers marins et les coraux là où elle repose.",
        "ecology_fr": "ce déchet métallique massif doit rejoindre la benne à métaux de la décharge.",
        "history_en": "this heavy plate was used to grill food over the fire.",
        "degradation_en": "the thick iron cast erodes very slowly over several decades in the sea.",
        "impact_en": "its high weight crushes seagrass beds and corals where it rests.",
        "ecology_en": "this massive metal waste must join the metal bin of the waste reception center."
    },
    {
        "name_fr": "Marmite en fonte émaillée",
        "name_en": "Enameled cast iron pot",
        "rarity": "rare",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "cette cocotte traditionnelle a vu sa couche d'émail s'écailler.",
        "degradation_fr": "l'émail protège le fer mais finit par sauter, exposant la fonte à la corrosion.",
        "impact_fr": "les morceaux d'émail tranchants agissent comme de petits éclats de verre.",
        "ecology_fr": "elle constitue un encombrant métallique lourd à recycler en déchetterie.",
        "history_en": "this traditional pot had its enamel layer peel off.",
        "degradation_en": "the enamel protects the iron but eventually chips off, exposing the cast iron to corrosion.",
        "impact_en": "the sharp pieces of enamel act like small glass shards.",
        "ecology_en": "it constitutes a heavy metallic bulky item to recycle at the waste reception center."
    },
    {
        "name_fr": "Plaque d'immatriculation",
        "name_en": "License plate",
        "rarity": "rare",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette plaque minéralogique métallique provient d'une voiture abandonnée.",
        "degradation_fr": "le film plastique réfléchissant se détache tandis que l'aluminium se corrode.",
        "impact_fr": "les rebords métalliques tordus peuvent être coupants pour la peau des poissons.",
        "ecology_fr": "le mélange intime d'aluminium et de plastique autocollant complique sa revalorisation.",
        "history_en": "this metallic registration plate comes from an abandoned car.",
        "degradation_en": "the reflective plastic film detaches while the aluminum corrodes.",
        "impact_en": "the bent metal edges can be sharp for the skin of fish.",
        "ecology_en": "the intimate mixture of aluminum and self-adhesive plastic complicates its recycling."
    },
    {
        "name_fr": "Pneu de voiture usé",
        "name_en": "Worn car tire",
        "rarity": "rare",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce pneu de berline a été jeté illégalement dans le cours d'eau.",
        "degradation_fr": "le caoutchouc vulcanisé et les fils d'acier durent plus de 1000 ans sans se décomposer.",
        "impact_fr": "il libère continuellement du zinc et des hydrocarbures aromatiques polycycliques toxiques.",
        "ecology_fr": "les pneus usés doivent impérativement être repris par un professionnel pour être broyés.",
        "history_en": "this sedan tire was illegally dumped into the watercourse.",
        "degradation_en": "the vulcanized rubber and steel wires last over 1000 years without decomposing.",
        "impact_en": "it continually releases zinc and toxic polycyclic aromatic hydrocarbons.",
        "ecology_en": "worn tires must absolutely be taken back by a professional to be shredded."
    },
    {
        "name_fr": "Batterie de vélo électrique",
        "name_en": "Electric bike battery",
        "rarity": "rare",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "cette batterie lithium-ion de vélo a perdu sa capacité de charge.",
        "degradation_fr": "le boîtier en aluminium s'oxyde tandis que les cellules internes fuient.",
        "impact_fr": "les solvants organiques et les sels de lithium empoisonnent l'eau environnante.",
        "ecology_fr": "c'est un déchet hautement inflammable qui exige un traitement spécialisé en déchetterie.",
        "history_en": "this bicycle lithium-ion battery lost its charge capacity.",
        "degradation_en": "the aluminum casing oxidizes while the internal cells leak.",
        "impact_en": "the organic solvents and lithium salts poison the surrounding water.",
        "ecology_en": "it is a highly flammable waste that requires specialized treatment at the waste reception center."
    },
    {
        "name_fr": "Extincteur vide",
        "name_en": "Empty fire extinguisher",
        "rarity": "rare",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": True, "vowel_en": True,
        "history_fr": "cet extincteur périmé a été jeté plutôt que d'être rechargé.",
        "degradation_fr": "la bouteille en acier épais résiste à la corrosion marine pendant des décennies.",
        "impact_fr": "les résidus de poudre d'extinction chimiques altèrent l'acidité locale de l'eau.",
        "ecology_fr": "c'est un récipient sous pression qui doit être déposé dans les points de collecte de la décharge.",
        "history_en": "this expired fire extinguisher was discarded rather than being refilled.",
        "degradation_en": "the thick steel cylinder resists marine corrosion for decades.",
        "impact_en": "the residues of chemical extinguishing powder alter the local acidity of water.",
        "ecology_en": "it is a pressurized vessel that must be deposited at the waste reception center collection points."
    },
    {
        "name_fr": "Siège auto pour bébé",
        "name_en": "Baby car seat",
        "rarity": "rare",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce siège auto en mousse et plastique dur est devenu hors d'usage.",
        "degradation_fr": "la structure en plastique rigide et les sangles synthétiques mettent des siècles à s'éroder.",
        "impact_fr": "les sangles peuvent ligoter et noyer les animaux marins curieux.",
        "ecology_fr": "ce déchet volumineux encombrant doit rejoindre la benne tout-venant de la décharge.",
        "history_en": "this foam and hard plastic car seat became unusable.",
        "degradation_en": "the rigid plastic structure and synthetic straps take centuries to erode.",
        "impact_en": "the straps can tie up and drown curious marine animals.",
        "ecology_en": "this bulky waste item must join the general waste bin of the waste reception center."
    },
    {
        "name_fr": "Trottinette pliable cassée",
        "name_en": "Broken folding scooter",
        "rarity": "rare",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette trottinette en aluminium s'est cassée au niveau de la charnière pliante.",
        "degradation_fr": "le cadre métallique résiste à la corrosion tandis que les roues en polyuréthane s'effritent.",
        "impact_fr": "les petites pièces mécaniques détachées peuvent être avalées par la faune marine.",
        "ecology_fr": "elle constitue un encombrant qui doit être déposé dans la benne à métaux de la décharge.",
        "history_en": "this aluminum scooter broke at the folding hinge.",
        "degradation_en": "the metal frame resists corrosion while the polyurethane wheels crumble.",
        "impact_en": "the small detached mechanical parts can be swallowed by marine life.",
        "ecology_en": "it constitutes a bulky item that must be deposited in the metal bin of the waste reception center."
    },

    # --- VERY RARE (15 items) ---
    {
        "name_fr": "Four de cuisine encastrable",
        "name_en": "Built-in kitchen oven",
        "rarity": "veryrare",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cet appareil de cuisson encombrant a été jeté lors d'une rénovation de cuisine.",
        "degradation_fr": "sa carcasse en acier inoxydable résiste très longtemps aux éléments extérieurs.",
        "impact_fr": "la laine de verre isolante se dissout et libère des fibres irritantes dans l'eau.",
        "ecology_fr": "ce gros électroménager doit rejoindre la filière de collecte des encombrants.",
        "history_en": "this bulky cooking appliance was discarded during a kitchen renovation.",
        "degradation_en": "its stainless steel casing resists the elements for a very long time.",
        "impact_en": "the insulating glass wool dissolves and releases irritating fibers in the water.",
        "ecology_en": "this large household appliance must join the bulky waste collection channel."
    },
    {
        "name_fr": "Lave-vaisselle HS",
        "name_en": "Broken dishwasher",
        "rarity": "veryrare",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce lave-vaisselle est tombé en panne après des années de bons services.",
        "degradation_fr": "sa cuve intérieure résiste à la corrosion tandis que la pompe électrique s'abîme.",
        "impact_fr": "les résidus de produits chimiques de lavage polluent l'eau avoisinante.",
        "ecology_fr": "les appareils volumineux exigent un dépôt en déchetterie pour le recyclage des métaux.",
        "history_en": "this dishwasher broke down after years of good service.",
        "degradation_en": "its internal tub resists corrosion while the electric pump decays.",
        "impact_en": "the residues of chemical washing products pollute the neighboring water.",
        "ecology_en": "voluminous appliances require deposit at the waste reception center for metal recycling."
    },
    {
        "name_fr": "Sèche-linge tambour bloqué",
        "name_en": "Tumble dryer with blocked drum",
        "rarity": "veryrare",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce sèche-linge a été jeté après le blocage mécanique de son tambour.",
        "degradation_fr": "les tôles peintes s'oxydent lentement tandis que la courroie en caoutchouc se dégrade.",
        "impact_fr": "les peluches de fibres synthétiques accumulées polluent l'eau sous forme de microfibres.",
        "ecology_fr": "cet appareil électroménager lourd doit être valorisé dans la filière métaux de la décharge.",
        "history_en": "this tumble dryer was discarded after the mechanical blockage of its drum.",
        "degradation_en": "the painted sheet metal slowly oxidizes while the rubber belt degrades.",
        "impact_en": "the accumulated synthetic fiber lint pollutes the water in the form of microfibers.",
        "ecology_en": "this heavy household appliance must be recycled in the metal channel of the waste reception center."
    },
    {
        "name_fr": "Chauffe-eau électrique",
        "name_en": "Electric water heater",
        "rarity": "veryrare",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "ce cumulus électrique a été remplacé après avoir fui à cause du calcaire.",
        "degradation_fr": "sa cuve interne en acier épais met plusieurs décennies à se corroder entièrement.",
        "impact_fr": "l'anode en magnésium se dissout dans l'eau tandis que la mousse isolante s'effrite.",
        "ecology_fr": "c'est un déchet métallique volumineux qui doit être traité en déchetterie.",
        "history_en": "this electric boiler was replaced after leaking due to lime scale.",
        "degradation_en": "its thick steel internal tank takes several decades to entirely corrode.",
        "impact_en": "the magnesium anode dissolves in water while the insulating foam crumbles.",
        "ecology_en": "it is a voluminous metallic waste that must be treated at the waste reception center."
    },
    {
        "name_fr": "Téléviseur écran plat brisé",
        "name_en": "Broken flat screen TV",
        "rarity": "veryrare",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce téléviseur LCD a reçu un choc fatal qui a brisé sa dalle en verre.",
        "degradation_fr": "les plastiques de la coque durent des siècles tandis que les cristaux liquides se répandent.",
        "impact_fr": "les cristaux liquides contiennent de l'indium et du mercure toxiques pour la faune.",
        "ecology_fr": "ce déchet électronique volumineux doit être déposé dans les encombrants de la décharge.",
        "history_en": "this LCD television received a fatal shock that broke its glass panel.",
        "degradation_en": "the plastics of the shell last for centuries while the liquid crystals leak out.",
        "impact_en": "the liquid crystals contain indium and mercury toxic to wildlife.",
        "ecology_en": "this voluminous electronic waste must be deposited in the bulky waste of the waste reception center."
    },
    {
        "name_fr": "Unité centrale d'ordinateur",
        "name_en": "Desktop computer tower",
        "rarity": "veryrare",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": True, "vowel_en": True,
        "history_fr": "ce boîtier d'ordinateur contient des cartes mères et des disques durs obsolètes.",
        "degradation_fr": "le châssis en tôle rouille tandis que les composants électroniques restent intactes.",
        "impact_fr": "les métaux lourds des soudures polluent l'eau et s'accumulent dans les poissons.",
        "ecology_fr": "elle contient des métaux précieux comme de l'or et de l'argent à recycler en décharge.",
        "history_en": "this computer case contains obsolete motherboards and hard drives.",
        "degradation_en": "the sheet metal chassis rusts while the electronic components remain intact.",
        "impact_en": "the heavy metals from the solderings pollute the water and accumulate in fish.",
        "ecology_en": "it contains precious metals like gold and silver to recycle at the waste reception center."
    },
    {
        "name_fr": "Imprimante jet d'encre",
        "name_en": "Inkjet printer",
        "rarity": "veryrare",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": True, "vowel_en": True,
        "history_fr": "cette imprimante a été jetée après l'obstruction de ses têtes d'impression.",
        "degradation_fr": "le plastique ABS de sa carcasse résiste au sel marin pendant 400 ans.",
        "impact_fr": "les cartouches d'encre restantes fuient et colorent l'eau de pigments toxiques.",
        "ecology_fr": "les imprimantes contiennent des moteurs pas-à-pas riches en métaux recyclables.",
        "history_en": "this printer was discarded after the clogging of its print heads.",
        "degradation_en": "the ABS plastic of its casing resists marine salt for 400 years.",
        "impact_en": "the remaining ink cartridges leak and color the water with toxic pigments.",
        "ecology_en": "printers contain stepper motors rich in recyclable metals."
    },
    {
        "name_fr": "Four à pizza de jardin",
        "name_en": "Garden pizza oven",
        "rarity": "veryrare",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce four extérieur en pierre reconstituée s'est fissuré avec le gel.",
        "degradation_fr": "la pierre et le béton réfractaire ne se dégradent pas mais s'érodent comme des galets.",
        "impact_fr": "son poids important écrase le substrat sédimentaire et perturbe la faune benthique.",
        "ecology_fr": "ce type de gravats doit rejoindre la benne à gravats inertes de la décharge.",
        "history_en": "this outdoor reconstituted stone oven cracked with the frost.",
        "degradation_en": "the stone and refractory concrete do not degrade but erode like pebbles.",
        "impact_en": "its high weight crushes the sedimentary substrate and disturbs benthic life.",
        "ecology_en": "this type of rubble must join the inert rubble bin of the waste reception center."
    },
    {
        "name_fr": "Carcasse de tondeuse à gazon",
        "name_en": "Lawnmower chassis",
        "rarity": "veryrare",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette tondeuse à essence a fini à l'eau après avoir été abandonnée dans un fossé.",
        "degradation_fr": "le carter en métal rouille lentement tandis que le réservoir plastique résiste.",
        "impact_fr": "les restes d'essence et d'huile moteur fuient et forment une nappe d'hydrocarbures.",
        "ecology_fr": "elle constitue un déchet polluant complexe à trier obligatoirement en déchetterie.",
        "history_en": "this petrol lawnmower ended up in the water after being abandoned in a ditch.",
        "degradation_en": "the metal deck rusts slowly while the plastic tank resists.",
        "impact_en": "the petrol and engine oil residues leak and form a hydrocarbon slick.",
        "ecology_en": "it constitutes a complex polluting waste item that must be sorted at the waste reception center."
    },
    {
        "name_fr": "Chariot élévateur manuel",
        "name_en": "Manual pallet jack",
        "rarity": "veryrare",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce transpalette en acier lourd a été jeté après la rupture de son vérin hydraulique.",
        "degradation_fr": "l'acier massif rouille lentement tandis que l'huile hydraulique se disperse.",
        "impact_fr": "les fuites d'huile hydraulique empoisonnent les poissons et étouffent la flore.",
        "ecology_fr": "ce déchet métallique massif doit rejoindre la benne à métaux de la décharge.",
        "history_en": "this heavy steel pallet jack was discarded after the rupture of its hydraulic cylinder.",
        "degradation_en": "the solid steel rusts slowly while the hydraulic oil disperses.",
        "impact_en": "hydraulic oil leaks poison fish and smother flora.",
        "ecology_en": "this massive metallic waste must join the metal bin of the waste reception center."
    },
    {
        "name_fr": "Baignoire en acrylique",
        "name_en": "Acrylic bathtub",
        "rarity": "veryrare",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "cette baignoire a été retirée lors de la rénovation d'une salle de bain.",
        "degradation_fr": "l'acrylique renforcé de fibre de verre est extrêmement résistant et dure des siècles.",
        "impact_fr": "sa grande surface incurvée peut piéger de grands poissons ou des tortues marines.",
        "ecology_fr": "ce déchet en plastique renforcé volumineux doit être jeté en déchetterie.",
        "history_en": "this bathtub was removed during a bathroom renovation.",
        "degradation_en": "the acrylic reinforced with fiberglass is extremely resistant and lasts for centuries.",
        "impact_en": "its large curved surface can trap large fish or sea turtles.",
        "ecology_en": "this bulky reinforced plastic waste item must be disposed of at the waste reception center."
    },
    {
        "name_fr": "Lavabo en céramique fendu",
        "name_en": "Cracked ceramic sink",
        "rarity": "veryrare",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce lavabo s'est brisé après la chute d'un objet lourd sur sa faïence.",
        "degradation_fr": "la céramique cuite est un matériau inerte qui résiste indéfiniment aux éléments.",
        "impact_fr": "les morceaux brisés ont des bords tranchants comme des rasoirs pour les animaux côtiers.",
        "ecology_fr": "ce type de gravats sanitaires doit rejoindre la benne appropriée à la décharge.",
        "history_en": "this sink broke after a heavy object fell on its earthenware.",
        "degradation_en": "fired ceramic is an inert material that resists the elements indefinitely.",
        "impact_en": "the broken pieces have razor-sharp edges for coastal animals.",
        "ecology_en": "this type of sanitary rubble must join the appropriate bin at the waste reception center."
    },
    {
        "name_fr": "Portail en fer forgé tordu",
        "name_en": "Bent wrought iron gate",
        "rarity": "veryrare",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce portail décoratif a été endommagé lors d'un accident de voiture.",
        "degradation_fr": "le fer forgé s'oxyde rapidement au contact de l'eau salée ou douce.",
        "impact_fr": "les pointes métalliques peuvent empaler les gros mammifères marins.",
        "ecology_fr": "ce déchet métallique lourd doit être déposé dans la benne métaux de la décharge.",
        "history_en": "this decorative gate was damaged during a car accident.",
        "degradation_en": "the wrought iron oxidizes quickly in contact with salty or fresh water.",
        "impact_en": "the metal spikes can impale large marine mammals.",
        "ecology_en": "this heavy metallic waste must be deposited in the metal bin of the waste reception center."
    },
    {
        "name_fr": "Radiateur en fonte",
        "name_en": "Cast iron radiator",
        "rarity": "veryrare",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce radiateur de chauffage central lourd a été remplacé par un modèle moderne.",
        "degradation_fr": "la fonte de fer épaisse s'oxyde lentement et met des décennies à disparaître.",
        "impact_fr": "son poids énorme écrase toute vie benthique sous son point de chute.",
        "ecology_fr": "il constitue un excellent gisement de fer recyclable en déchetterie.",
        "history_en": "this heavy central heating radiator was replaced by a modern model.",
        "degradation_en": "the thick iron cast oxidizes slowly and takes decades to disappear.",
        "impact_en": "its enormous weight crushes all benthic life beneath its drop point.",
        "ecology_en": "it constitutes an excellent source of recyclable iron at the waste reception center."
    },
    {
        "name_fr": "Poutre en acier (IPN)",
        "name_en": "I-beam steel girder",
        "rarity": "veryrare",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "cette poutre de soutien en acier provient de la démolition d'un bâtiment.",
        "degradation_fr": "l'acier de construction se corrode doucement au fil des ans dans le cours d'eau.",
        "impact_fr": "sa structure métallique rigide bloque la libre circulation des gros poissons.",
        "ecology_fr": "les profilés métalliques de construction doivent être recyclés dans la filière métaux de la décharge.",
        "history_en": "this steel support beam originates from a building demolition.",
        "degradation_en": "the construction steel corrodes slowly over the years in the watercourse.",
        "impact_en": "its rigid metal structure blocks the free movement of large fish.",
        "ecology_en": "construction metal profiles must be recycled in the metal channel of the waste reception center."
    },

    # --- EPIC (11 items) ---
    {
        "name_fr": "Moteur de voiture diesel",
        "name_en": "Diesel car engine",
        "rarity": "epic",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "ce bloc moteur en fonte a été jeté illégalement après démontage.",
        "degradation_fr": "le bloc de fonte s'oxyde très lentement tandis que les durites en caoutchouc sèchent.",
        "impact_fr": "les résidus d'huile noire de vidange asphyxient la faune locale.",
        "ecology_fr": "c'est une pollution majeure qui exige un traitement dépolluant en déchetterie.",
        "history_en": "this cast iron engine block was illegally dumped after dismantling.",
        "degradation_en": "the iron block oxidizes very slowly while the rubber hoses dry out.",
        "impact_en": "black engine oil residues suffocate the local wildlife.",
        "ecology_en": "it is a major pollution that requires a depolluting treatment at the waste reception center."
    },
    {
        "name_fr": "Pont de voiture (essieu)",
        "name_en": "Car axle and differential",
        "rarity": "epic",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": True, "vowel_en": True,
        "history_fr": "cet essieu arrière métallique de voiture a fini sa course dans l'eau.",
        "degradation_fr": "l'acier massif résiste à la corrosion et met plus de cent ans à se dégrader.",
        "impact_fr": "le différentiel contient de l'huile d'engrenage toxique qui fuit.",
        "ecology_fr": "les déchets automobiles complexes doivent être recyclés par des centres agréés.",
        "history_en": "this metal car rear axle ended up in the water.",
        "degradation_en": "the solid steel resists corrosion and takes over a hundred years to degrade.",
        "impact_en": "the differential contains toxic gear oil that leaks.",
        "ecology_en": "complex automotive waste must be recycled by authorized centers."
    },
    {
        "name_fr": "Carcasse de caravane pliante",
        "name_en": "Folding camper shell",
        "rarity": "epic",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette vieille caravane de camping a été abandonnée près de la berge.",
        "degradation_fr": "sa structure mêlant résine, aluminium et bois pourrit lentement.",
        "impact_fr": "les solvants des colles et les résines plastiques empoisonnent l'eau.",
        "ecology_fr": "les véhicules hors d'usage doivent être dépollués et broyés en décharge.",
        "history_en": "this old camping caravan was abandoned near the riverbank.",
        "degradation_en": "its structure mixing resin, aluminum and wood rots slowly.",
        "impact_en": "glues solvents and plastic resins poison the water.",
        "ecology_en": "end-of-life vehicles must be depolluted and shredded at the waste reception center."
    },
    {
        "name_fr": "Réservoir de carburant industriel",
        "name_en": "Industrial fuel tank",
        "rarity": "epic",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "cette citerne servait à stocker du fioul lourd ou du gazole.",
        "degradation_fr": "les parois en acier rouillent lentement et finissent par céder.",
        "impact_fr": "les résidus de fioul créent une marée noire locale destructrice.",
        "ecology_fr": "ce réservoir exige un dégazage et une dépollution stricte en déchetterie.",
        "history_en": "this cistern was used to store heavy fuel oil or diesel.",
        "degradation_en": "the steel walls slowly rust and eventually give way.",
        "impact_en": "fuel residues create a destructive local oil spill.",
        "ecology_en": "this tank requires degassing and strict depollution at the waste reception center."
    },
    {
        "name_fr": "Éolienne miniature brisée",
        "name_en": "Broken mini wind turbine",
        "rarity": "epic",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": True, "vowel_en": False,
        "history_fr": "cette petite éolienne domestique s'est envolée lors d'une tempête.",
        "degradation_fr": "les pales en fibre de carbone ne se dégradent pas et l'alternateur s'oxyde.",
        "impact_fr": "le bobinage de cuivre et l'aimant néodyme s'altèrent, libérant des toxines.",
        "ecology_fr": "les matériaux composites exigent un traitement en déchetterie.",
        "history_en": "this small domestic wind turbine blew away during a storm.",
        "degradation_en": "the carbon fiber blades do not degrade and the alternator oxidizes.",
        "impact_en": "the copper winding and neodymium magnet decay, releasing toxins.",
        "ecology_en": "composite materials require treatment at the waste reception center."
    },
    {
        "name_fr": "Pelle mécanique (bras)",
        "name_en": "Excavator arm",
        "rarity": "epic",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "ce bras d'excavatrice en acier de chantier s'est rompu sous l'effort.",
        "degradation_fr": "l'acier trempé massif résiste à la rouille pendant plus d'un siècle.",
        "impact_fr": "les vérins hydrauliques fuient et déversent de l'huile minérale toxique.",
        "ecology_fr": "ce type de ferraille lourde de chantier doit être traité par un ferrailleur en décharge.",
        "history_en": "this excavator arm made of construction steel broke under stress.",
        "degradation_en": "the solid tempered steel resists rust for over a century.",
        "impact_en": "the hydraulic cylinders leak and pour toxic mineral oil.",
        "ecology_en": "this type of heavy construction scrap metal must be treated by a scrap dealer at the waste reception center."
    },
    {
        "name_fr": "Ancre de marine à jas",
        "name_en": "Stock anchor",
        "rarity": "epic",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": True, "vowel_en": False,
        "history_fr": "cette ancre traditionnelle en fer forgé servait à stabiliser un navire.",
        "degradation_fr": "le fer s'oxyde très lentement au fond de l'eau en formant de la limonite.",
        "impact_fr": "elle repose au fond, modifiant le courant local et le substrat.",
        "ecology_fr": "les ancres marines perdues sont des reliques métalliques à recycler en déchetterie.",
        "history_en": "this traditional wrought iron anchor was used to stabilize a ship.",
        "degradation_en": "the iron oxidizes very slowly at the bottom of the water, forming limonite.",
        "impact_en": "it rests at the bottom, modifying the local current and substrate.",
        "ecology_en": "lost marine anchors are metallic relics to recycle at the waste reception center."
    },
    {
        "name_fr": "Treuil de chalutier",
        "name_en": "Trawler winch",
        "rarity": "epic",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce treuil métallique servait à remonter les filets de pêche.",
        "degradation_fr": "le bloc de fer massif s'oxyde très lentement dans l'eau salée.",
        "impact_fr": "le câble métallique en acier tressé s'effiloche et piège les poissons.",
        "ecology_fr": "les vieux treuils doivent être démontés et recyclés pour leur ferraille.",
        "history_en": "this metal winch was used to haul in fishing nets.",
        "degradation_en": "the solid iron block oxidizes very slowly in saltwater.",
        "impact_en": "the braided steel wire rope frays and traps fish.",
        "ecology_en": "old winches must be dismantled and recycled for their scrap metal."
    },
    {
        "name_fr": "Carcasse de jet-ski",
        "name_en": "Jet-ski hull",
        "rarity": "epic",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette coque de jet-ski en résine a été abandonnée après un naufrage.",
        "degradation_fr": "la fibre de verre et le polyester ne se décomposent jamais dans l'eau.",
        "impact_fr": "la carcasse flotte à la dérive et heurte les animaux marins.",
        "ecology_fr": "les coques en composite résine doivent être traitées par des filières spécialisées.",
        "history_en": "this resin jet-ski hull was abandoned after a shipwreck.",
        "degradation_en": "the fiberglass and polyester never decompose in water.",
        "impact_en": "the hull drifts along and collides with marine animals.",
        "ecology_en": "resin composite hulls must be treated by specialized channels."
    },
    {
        "name_fr": "Hélice de cargo en bronze",
        "name_en": "Bronze cargo propeller",
        "rarity": "epic",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": True, "vowel_en": False,
        "history_fr": "cette grande hélice en bronze s'est détachée de l'arbre d'un cargo.",
        "degradation_fr": "le bronze est très résistant à la corrosion marine et dure des millénaires.",
        "impact_fr": "sa masse écrase la flore et modifie les courants de fond locaux.",
        "ecology_fr": "le bronze est un alliage de cuivre précieux à recycler absolument en déchetterie.",
        "history_en": "this large bronze propeller detached from a cargo ship shaft.",
        "degradation_en": "bronze is highly resistant to marine corrosion and lasts for millennia.",
        "impact_en": "its mass crushes flora and modifies local bottom currents.",
        "ecology_en": "bronze is a valuable copper alloy to absolutely recycle at the waste reception center."
    },
    {
        "name_fr": "Cabine de tracteur agricole",
        "name_en": "Farm tractor cab",
        "rarity": "epic",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette cabine métallique provient d'un vieux tracteur agricole hors d'usage.",
        "degradation_fr": "la tôle s'oxyde lentement tandis que les vitres en verre trempé durent.",
        "impact_fr": "les vitres brisées représentent un risque de coupure mortelle pour la faune.",
        "ecology_fr": "ce déchet agricole massif doit être revalorisé dans les métaux lourds en décharge.",
        "history_en": "this metal cab originates from a worn-out agricultural tractor.",
        "degradation_en": "the sheet metal oxidizes slowly while the tempered glass windows last.",
        "impact_en": "the broken windows present a mortal cut risk for wildlife.",
        "ecology_en": "this massive agricultural waste must be recycled with heavy metals at the waste reception center."
    },

    # --- LEGENDARY (6 items) ---
    {
        "name_fr": "Ailes d'avion léger",
        "name_en": "Light aircraft wings",
        "rarity": "legendary",
        "bin": "decharge",
        "gender": "F", "plural": True, "vowel_fr": True, "vowel_en": False,
        "history_fr": "ces ailes d'aluminium proviennent d'un avion léger de tourisme accidenté.",
        "degradation_fr": "l'aluminium aéronautique s'oxyde doucement en créant une couche blanche de passivation.",
        "impact_fr": "leur grande surface bloque le passage de la faune aquatique sur le fond.",
        "ecology_fr": "l'aluminium aéronautique recyclé possède une grande valeur industrielle en décharge.",
        "history_en": "these aluminum wings originate from a crashed light tourist aircraft.",
        "degradation_en": "the aeronautical aluminum oxidizes slowly, creating a white passivation layer.",
        "impact_en": "their large surface area blocks the passage of aquatic life on the bed.",
        "ecology_en": "recycled aeronautical aluminum has high industrial value at the waste reception center."
    },
    {
        "name_fr": "Wagon de train de marchandises",
        "name_en": "Train freight car",
        "rarity": "legendary",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce wagon de train de marchandises a déraillé et a chu dans le fleuve.",
        "degradation_fr": "l'acier ferroviaire massif s'oxyde très lentement sur des siècles.",
        "impact_fr": "sa structure modifie le cours d'eau et piège les poissons géants.",
        "ecology_fr": "le recyclage de cette masse d'acier permet de produire des tonnes de métal neuf.",
        "history_en": "this train freight car derailed and fell into the river.",
        "degradation_en": "the massive railway steel oxidizes very slowly over centuries.",
        "impact_en": "its structure modifies the water flow and traps giant fish.",
        "ecology_en": "recycling this mass of steel allows the production of tons of new metal."
    },
    {
        "name_fr": "Locomotive de manœuvre",
        "name_en": "Shunting locomotive",
        "rarity": "legendary",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette petite locomotive diesel servait à déplacer les wagons en gare.",
        "degradation_fr": "sa structure métallique rouille lentement tandis que le moteur fuit.",
        "impact_fr": "les fuites massives d'hydrocarbures polluent durablement le cours d'eau.",
        "ecology_fr": "les locomotives exigent une dépollution complète avant recyclage en décharge.",
        "history_en": "this small diesel locomotive was used to shunt railcars in the station.",
        "degradation_en": "its metal structure rusts slowly while the engine leaks.",
        "impact_en": "massive hydrocarbon leaks pollute the watercourse for a long time.",
        "ecology_en": "locomotives require complete depollution before recycling at the waste reception center."
    },
    {
        "name_fr": "Carcasse d'hélicoptère civil",
        "name_en": "Civilian helicopter fuselage",
        "rarity": "legendary",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cet hélicoptère s'est écrasé dans l'eau lors d'une opération de sauvetage.",
        "degradation_fr": "le fuselage en aluminium et les pales en composite ne s'érodent pas.",
        "impact_fr": "le réservoir restant présente un risque de fuite de kérosène toxique.",
        "ecology_fr": "les restes d'aéronefs doivent être évacués et dépollués par des professionnels.",
        "history_en": "this helicopter crashed in the water during a rescue operation.",
        "degradation_en": "the aluminum fuselage and composite blades do not erode.",
        "impact_en": "the remaining fuel tank presents a risk of toxic kerosene leakage.",
        "ecology_en": "aircraft remains must be evacuated and depolluted by professionals."
    },
    {
        "name_fr": "Générateur de centrale (rotor)",
        "name_en": "Power plant generator rotor",
        "rarity": "legendary",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce rotor de turbine électrique géant a été mis au rebut après usure.",
        "degradation_fr": "l'acier allié s'oxyde très lentement au fil des siècles.",
        "impact_fr": "sa masse colossale écrase le biotope sédimentaire sur des dizaines de mètres carrés.",
        "ecology_fr": "il contient des bobinages de cuivre pur de très haute valeur industrielle.",
        "history_en": "this giant electric turbine rotor was discarded after wear and tear.",
        "degradation_en": "the alloy steel oxidizes very slowly over the centuries.",
        "impact_en": "its colossal mass crushes the sedimentary biotope over dozens of square meters.",
        "ecology_en": "it contains pure copper windings of very high industrial value."
    },
    {
        "name_fr": "Radar maritime géant",
        "name_en": "Giant marine radar",
        "rarity": "legendary",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce dôme de radar provient d'un navire militaire déclassé.",
        "degradation_fr": "le dôme en composite de verre résiste tandis que l'électronique s'altère.",
        "impact_fr": "les composants internes contiennent des métaux lourds hautement polluants.",
        "ecology_fr": "ce radar exige un démantèlement complexe en déchetterie spécialisée.",
        "history_en": "this radar dome originates from a decommissioned military vessel.",
        "degradation_en": "the glass composite dome resists while the electronics decay.",
        "impact_en": "the internal components contain highly polluting heavy metals.",
        "ecology_en": "this radar requires complex dismantling at a specialized waste reception center."
    },

    # --- MYTHICAL (6 items) ---
    {
        "name_fr": "Module lunaire (réplique)",
        "name_en": "Lunar module replica",
        "rarity": "mythical",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "cette réplique grandeur nature du module lunaire Apollo a été jetée par un musée.",
        "degradation_fr": "l'aluminium brillant et les feuilles plastifiées dorées résistent au temps.",
        "impact_fr": "sa structure fine peut piéger de grands poissons à l'intérieur de sa cabine.",
        "ecology_fr": "ce déchet historique en résine et métaux doit être démonté et trié en décharge.",
        "history_en": "this life-size replica of the Apollo lunar module was discarded by a museum.",
        "degradation_en": "the shiny aluminum and golden plasticized sheets resist time.",
        "impact_en": "its fine structure can trap large fish inside its cabin.",
        "ecology_en": "this historical resin and metal waste must be dismantled and sorted at the dump."
    },
    {
        "name_fr": "Carcasse de char d'assaut",
        "name_en": "Tank hull",
        "rarity": "mythical",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce char de combat blindé a été coulé lors d'exercices militaires passés.",
        "degradation_fr": "l'acier de blindage épais rouille lentement au fond des eaux.",
        "impact_fr": "sa structure massive modifie les courants côtiers et la sédimentation.",
        "ecology_fr": "il constitue un récif artificiel métallique qui doit pourtant être surveillé.",
        "history_en": "this armored combat tank was sunk during past military exercises.",
        "degradation_en": "the thick armor steel rusts slowly at the bottom of the waters.",
        "impact_en": "its massive structure modifies coastal currents and sedimentation.",
        "ecology_en": "it constitutes an artificial metallic reef that must nevertheless be monitored."
    },
    {
        "name_fr": "Tourelle de cuirassé",
        "name_en": "Battleship turret",
        "rarity": "mythical",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette tourelle en acier blindé provient d'un cuirassé de la Seconde Guerre mondiale.",
        "degradation_fr": "l'acier cémenté massif s'oxyde très lentement au fil des siècles marins.",
        "impact_fr": "son poids de plusieurs centaines de tonnes écrase le relief marin local.",
        "ecology_fr": "le traitement d'un tel bloc de métal exige des moyens de levage militaires.",
        "history_en": "this armored steel turret originates from a Second World War battleship.",
        "degradation_en": "the massive case-hardened steel oxidizes very slowly over marine centuries.",
        "impact_en": "its weight of several hundred tons crushes the local marine relief.",
        "ecology_en": "the treatment of such a block of metal requires military lifting equipment."
    },
    {
        "name_fr": "Mine magnétique (WW2)",
        "name_en": "Magnetic mine (WW2)",
        "rarity": "mythical",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette mine magnétique marine de la Seconde Guerre mondiale repose au fond.",
        "degradation_fr": "le métal extérieur s'oxyde tandis que l'explosif intérieur reste stable.",
        "impact_fr": "elle présente un risque d'explosion majeur en cas de manipulation imprudente.",
        "ecology_fr": "elle exige l'intervention urgente de démineurs de la marine nationale.",
        "history_en": "this Second World War magnetic marine mine rests on the seabed.",
        "degradation_en": "the outer metal oxidizes while the internal explosive remains stable.",
        "impact_en": "it presents a major explosion risk in case of imprudent manipulation.",
        "ecology_en": "it requires the urgent intervention of navy bomb disposal experts."
    },
    {
        "name_fr": "Satellite de télécom HS",
        "name_en": "Broken telecom satellite",
        "rarity": "mythical",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce satellite de télécommunications a échoué dans l'océan lors de sa rentrée.",
        "degradation_fr": "les alliages d'aluminium et de titane durent des millénaires dans l'eau.",
        "impact_fr": "les composants radioactifs ou toxiques de ses batteries polluent la zone.",
        "ecology_fr": "les débris spatiaux doivent être récupérés pour éviter la pollution des fonds marins.",
        "history_en": "this telecommunications satellite crashed into the ocean during its reentry.",
        "degradation_en": "the aluminum and titanium alloys last for millennia in the water.",
        "impact_en": "the radioactive or toxic components of its batteries pollute the area.",
        "ecology_en": "space debris must be recovered to prevent pollution of the seabed."
    },
    {
        "name_fr": "Fragment de station Mir",
        "name_en": "Piece of the Mir station",
        "rarity": "mythical",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce débris provient de la célèbre station spatiale russe Mir retombée sur Terre.",
        "degradation_fr": "les isolants thermiques et les panneaux métalliques résistent à l'érosion.",
        "impact_fr": "les arêtes métalliques brûlées lors de la rentrée coupent les poissons.",
        "ecology_fr": "sa récupération permet de préserver l'intégrité de la zone de retombée marine.",
        "history_en": "this debris originates from the famous Russian space station Mir that fell back to Earth.",
        "degradation_en": "the thermal insulators and metal panels resist erosion.",
        "impact_en": "the metal edges burned during reentry cut fish.",
        "ecology_en": "its recovery helps preserve the integrity of the marine splashdown zone."
    },

    # --- DIVIN (4 items) ---
    {
        "name_fr": "Capsule spatiale Apollo",
        "name_en": "Apollo space capsule",
        "rarity": "divin",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "cette capsule d'exploration spatiale a amerri dans l'océan et s'est perdue.",
        "degradation_fr": "son bouclier thermique et ses alliages exotiques durent des dizaines de milliers d'années.",
        "impact_fr": "sa masse solide crée une obstruction physique permanente pour la vie marine.",
        "ecology_fr": "elle constitue un trésor historique qui doit être extrait pour être restauré.",
        "history_en": "this space exploration capsule splashed down in the ocean and was lost.",
        "degradation_en": "its heat shield and exotic alloys last for tens of thousands of years.",
        "impact_en": "its solid mass creates a permanent physical obstruction for marine life.",
        "ecology_en": "it constitutes a historical treasure that must be extracted to be restored."
    },
    {
        "name_fr": "Rover martien (prototype)",
        "name_en": "Martian rover prototype",
        "rarity": "divin",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "ce prototype de véhicule martien a été jeté lors d'essais secrets en milieu humide.",
        "degradation_fr": "le titane et le carbone composites résistent à toute forme d'oxydation.",
        "impact_fr": "la batterie nucléaire inerte peut libérer des éléments toxiques si elle est percée.",
        "ecology_fr": "sa structure complexe exige un démantèlement de haute technologie en décharge.",
        "history_en": "this prototype martian vehicle was discarded during secret wet tests.",
        "degradation_en": "the titanium and composite carbon resist any form of oxidation.",
        "impact_en": "the inert nuclear battery can release toxic elements if punctured.",
        "ecology_en": "its complex structure requires high-technology dismantling at the dump."
    },
    {
        "name_fr": "Cœur de réacteur inerte",
        "name_en": "Inert reactor core",
        "rarity": "divin",
        "bin": "decharge",
        "gender": "M", "plural": False, "vowel_fr": False, "vowel_en": True,
        "history_fr": "ce conteneur abritait un cœur de réacteur expérimental désormais inerte.",
        "degradation_fr": "le cuvelage en plomb et acier résiste aux pressions sous-marines et au sel.",
        "impact_fr": "toute fissure provoquerait une fuite de matériaux lourds dans l'eau.",
        "ecology_fr": "ce déchet nucléaire inerte doit faire l'objet d'une surveillance militaire stricte.",
        "history_en": "this container housed an experimental reactor core, now inert.",
        "degradation_en": "the lead and steel casing resists underwater pressures and salt.",
        "impact_en": "any crack would cause heavy material leakage into the water.",
        "ecology_en": "this inert nuclear waste must be subject to strict military monitoring."
    },
    {
        "name_fr": "Sonde Pioneer (plaque)",
        "name_en": "Pioneer probe plaque",
        "rarity": "divin",
        "bin": "decharge",
        "gender": "F", "plural": False, "vowel_fr": False, "vowel_en": False,
        "history_fr": "cette plaque dorée en aluminium anodisé provient d'une sonde spatiale perdue.",
        "degradation_fr": "le revêtement en or protège le message gravé de toute altération chimique.",
        "impact_fr": "sa surface brillante attire les grands requins qui peuvent la mordre.",
        "ecology_fr": "c'est un artefact inestimable représentant l'humanité à conserver précieusement.",
        "history_en": "this anodized aluminum golden plaque comes from a lost space probe.",
        "degradation_en": "the gold plating protects the engraved message from any chemical alteration.",
        "impact_en": "its shiny surface attracts large sharks that can bite it.",
        "ecology_en": "it is an invaluable artifact representing humanity to be carefully preserved."
    }
]

def make_fun_fact_fr(name, bin_name, history, degradation, impact, ecology):
    fact = f"L'objet nommé '{name}' est un déchet aquatique. {history} Lorsqu'il se retrouve immergé dans l'eau, il subit un processus de dégradation spécifique : {degradation} Sur le plan environnemental, ce déchet représente une menace majeure car {impact} Enfin, les conséquences écologiques à long terme sont préoccupantes, car {ecology} Il est donc primordial de trier ce déchet dans la poubelle '{bin_name}' pour limiter la pollution de nos cours d'eau."
    return fact

def make_fun_fact_en(name, bin_name, history, degradation, impact, ecology):
    fact = f"The object named '{name}' is aquatic waste. {history} When it is submerged in water, it undergoes a specific degradation process: {degradation} Environmentally, this waste represents a major threat because {impact} Finally, the long-term ecological consequences are concerning, because {ecology} It is therefore essential to sort this waste into the '{bin_name}' bin to limit the pollution of our waterways."
    return fact

def get_desc_fr(name, gender, plural, vowel):
    art_indef = "Des" if plural else ("Une" if gender == "F" else "Un")
    art_def = "Les" if plural else ("La" if gender == "F" else ("L'" if vowel else "Le"))
    adj_dem = "Ces" if plural else ("Cette" if gender == "F" else ("Cet" if vowel else "Ce"))
    
    ag_s = "es" if (gender == "F" and plural) else ("s" if plural else ("e" if gender == "F" else ""))
    verb_be = "sont" if plural else "est"
    noun_debris = "des débris informes" if plural else "un débris informe"
    adj_diger = "digérés" if plural else ("digérée" if gender == "F" else "digéré")
    
    badly_damaged = [
        f"🔴 {art_indef} {name} complètement déchiqueté{ag_s} et recouvert{ag_s} d'algues visqueuses.",
        f"🔴 {adj_dem} {name} ne {verb_be} plus qu'{noun_debris}, à moitié {adj_diger} par le sel.",
        f"🔴 Des fragments de {adj_dem.lower()} {name} flottent tristement, méconnaissables et décolorés par le soleil."
    ]
    
    adj_fendu = "fendus" if plural else ("fendue" if gender == "F" else "fendu")
    adj_deform = "déformés" if plural else ("déformée" if gender == "F" else "déformé")
    adj_rempli = "remplis" if plural else ("remplie" if gender == "F" else "rempli")
    adj_abime = "abîmés" if plural else ("abîmée" if gender == "F" else "abîmé")
    
    damaged = [
        f"🟠 {art_indef} {name} {adj_fendu} et {adj_deform}, {adj_rempli}{ag_s} d'eau de mer et de sédiments.",
        f"🟠 {adj_dem} {name} montre{'' if not plural else 'nt'} des signes évidents de détérioration, avec des fissures ou des cassures.",
        f"🟠 {art_indef} {name} {adj_abime} par les vagues, présentant des traces importantes d'usure."
    ]
    
    adj_entier = "entiers" if plural else ("entière" if gender == "F" else "entier")
    adj_couvert = "couverts" if plural else ("couverte" if gender == "F" else "couvert")
    adj_decolor = "décolorés" if plural else ("décolorée" if gender == "F" else "décoloré")
    adj_use = "usés" if plural else ("usée" if gender == "F" else "usé")
    adj_usage = "usagés" if plural else ("usagée" if gender == "F" else "usagé")
    verb_derive = "dérivent" if plural else "dérive"
    
    worn = [
        f"🟡 {art_indef} {name} presque {adj_entier}, bien que {adj_couvert} d'une fine couche de limon et d'algues.",
        f"🟡 {adj_dem} {name} a{'' if not plural else 'ont'} perdu de sa superbe, {adj_decolor} par le soleil et {adj_use} par le frottement du sable.",
        f"🟡 {art_indef} {name} {adj_usage} qui {verb_derive} au gré des courants marins."
    ]
    
    return {
        "badly damaged": badly_damaged,
        "damaged": damaged,
        "worn": worn
    }

def get_desc_en(name, plural, vowel):
    art_indef = "Some" if plural else ("An" if vowel else "A")
    adj_dem = "These" if plural else "This"
    verb_be = "are" if plural else "is"
    verb_have = "have" if plural else "has"
    noun_debris = "fragments of debris" if plural else "a piece of debris"
    verb_drift = "drift" if plural else "drifts"
    
    badly_damaged = [
        f"🔴 {art_indef} {name} completely shredded and covered with slimy seaweed.",
        f"🔴 {adj_dem} {name} {verb_be} nothing but {noun_debris}, half-digested by the salt.",
        f"🔴 Shreds of {adj_dem.lower()} {name} float sadly, unrecognizable and faded by the sun."
    ]
    
    damaged = [
        f"🟠 {art_indef} {name}, cracked and deformed, filled with seawater and sediment.",
        f"🟠 {adj_dem} {name} show{'' if plural else 's'} clear signs of deterioration, with deep cracks.",
        f"🟠 {art_indef} {name} damaged by the waves, showing significant wear."
    ]
    
    worn = [
        f"🟡 {art_indef} {name} almost intact, although covered with a fine layer of silt and algae.",
        f"🟡 {adj_dem} {name} {verb_have} lost {'' if plural else 'its '}lustre, faded by the sun and worn by the friction of sand.",
        f"🟡 {art_indef} {name} worn out, gently {verb_drift} with the marine currents."
    ]
    
    return {
        "badly damaged": badly_damaged,
        "damaged": damaged,
        "worn": worn
    }

def process_file(file_path, is_en=False):
    print(f"Processing {file_path}...")
    with open(file_path, "r", encoding="utf-8") as f:
        data = json.load(f)
    
    # Check max ID in existing
    max_id = 0
    for key, items in data.items():
        for item in items:
            if item.get("id", 0) > max_id:
                max_id = item["id"]
    
    print(f"Max ID found: {max_id}")
    next_id = max_id + 1 if max_id > 0 else 143 # start from 143 if clean
    if max_id < 142:
        # Gaps check: if it is less than 142, let's keep next_id as 143 to start
        next_id = 143

    # Add items to their respective lists
    added_count = 0
    for item_data in new_items_data:
        rarity = item_data["rarity"]
        bin_name = item_data["bin"]
        name_fr = item_data["name_fr"]
        name_en = item_data["name_en"]
        gender = item_data["gender"]
        plural = item_data["plural"]
        vowel_fr = item_data["vowel_fr"]
        vowel_en = item_data["vowel_en"]
        
        bonus, malus = NOTORIETY_VALUES[rarity]
        
        # Build descriptions
        if is_en:
            desc = get_desc_en(name_en, plural, vowel_en)
            fact = make_fun_fact_en(name_en, bin_name, item_data["history_en"], item_data["degradation_en"], item_data["impact_en"], item_data["ecology_en"])
        else:
            desc = get_desc_fr(name_fr, gender, plural, vowel_fr)
            fact = make_fun_fact_fr(name_fr, bin_name, item_data["history_fr"], item_data["degradation_fr"], item_data["impact_fr"], item_data["ecology_fr"])
        
        # Ensure length constraint
        if len(fact) < 500:
            print(f"WARNING: Fact for {name_fr} ({len(fact)} chars) is under 500!")
            
        json_item = {
            "id": next_id,
            "name": name_fr, # both files use name_fr for name key to keep matching identical
            "size_min": 0.0,
            "size_mean": 0.0,
            "size_sigma": 0.0,
            "force_pristine": None,
            "force_state": None,
            "descriptions": desc,
            "fun_fact": fact,
            "bin": bin_name,
            "recycling_notoriety_bonus": bonus,
            "recycling_notoriety_malus": malus
        }
        
        if rarity not in data:
            data[rarity] = []
            
        data[rarity].append(json_item)
        next_id += 1
        added_count += 1
        
    # Write back
    with open(file_path, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=4, ensure_ascii=False)
    print(f"Added {added_count} items to {file_path}. New max ID is {next_id - 1}")

if __name__ == "__main__":
    if len(new_items_data) != 100:
        print(f"ERROR: You must define exactly 100 items! Currently defined: {len(new_items_data)}")
    else:
        # Check lengths of all fun facts
        all_ok = True
        for idx, item in enumerate(new_items_data):
            bin_name = item["bin"]
            fact_fr = make_fun_fact_fr(item["name_fr"], bin_name, item["history_fr"], item["degradation_fr"], item["impact_fr"], item["ecology_fr"])
            fact_en = make_fun_fact_en(item["name_en"], bin_name, item["history_en"], item["degradation_en"], item["impact_en"], item["ecology_en"])
            if len(fact_fr) < 500:
                print(f"ERROR FR: Item {idx} '{item['name_fr']}' has fact length {len(fact_fr)} (< 500)")
                all_ok = False
            if len(fact_en) < 500:
                print(f"ERROR EN: Item {idx} '{item['name_en']}' has fact length {len(fact_en)} (< 500)")
                all_ok = False
        
        if all_ok:
            process_file(JUNK_FR_PATH, is_en=False)
            process_file(JUNK_EN_PATH, is_en=True)
            print("Successfully processed both files!")
        else:
            print("Failed validation, not writing files.")
