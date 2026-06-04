# -*- coding: utf-8 -*-
from update_junk_util import apply_updates

updates_fr = [
    {
        "id": 193,
        "descriptions": {
            "badly damaged": [
                "🔴 Une plaque de cuisson en fonte complètement brisée en plusieurs blocs lourds et rouillés.",
                "🔴 Cet élément de cuisine montre une surface de fonte s'effritant sous forme d'écailles d'oxyde de fer.",
                "🔴 Une plaque métallique massive à demi ensevelie sous les galets et la vase noire argileuse."
            ],
            "damaged": [
                "🟠 Une plaque de cuisson présentant des fissures profondes traversant sa structure métallique.",
                "🟠 Cet objet montre des dépôts importants de calcaire blanc et des manettes arrachées.",
                "🟠 Une plaque en fonte déformée par le poids d'autres débris de fond, couverte de vase."
            ],
            "worn": [
                "🟡 Une plaque de cuisson en fonte entière bien que sale, reposant à plat sur le fond.",
                "🟡 Cet élément de fonte a conservé son aspect rectangulaire malgré des éraflures superficielles.",
                "🟡 Une plaque de cuisson usagée jetée récemment, encroûtée d'un léger voile de limon."
            ]
        },
        "fun_fact": "La plaque de cuisson en fonte est un équipement de cuisine très lourd fabriqué en fonte de fer grise. Abandonné dans l'eau, il présente un risque physique d'écrasement pour la faune benthique qui vit sur les sédiments de fond. Bien que le fer s'oxyde lentement pour former de la rouille inoffensive, sa mèche ou ses bords fracturés peuvent blesser les poissons. De plus, la fonte mettra de nombreuses décennies à se corroder entièrement. Elle doit être déposée dans la benne à métaux de la décharge."
    },
    {
        "id": 194,
        "descriptions": {
            "badly damaged": [
                "🔴 Une marmite en fonte dont l'émail est complètement écaillé et le fond percé par la corrosion.",
                "🔴 Ce récipient lourd montre une anse métallique cassée et une structure encroûtée de vase noire fétide.",
                "🔴 Des débris de fonte émaillée brisés en éclats pointus et incrustés de calcaire et de coquillages."
            ],
            "damaged": [
                "🟠 Une marmite en fonte présentant de larges éclats sur son revêtement d'émail coloré.",
                "🟠 Ce récipient montre des traces prononcées de rouille brune sur ses flancs et une anse tordue.",
                "🟠 Une marmite déformée par des chocs violents contre des rochers, contenant du sable fin."
            ],
            "worn": [
                "🟡 Une marmite en fonte émaillée entière bien que sale, avec ses deux anses encore en place.",
                "🟡 Ce récipient a conservé sa couleur d'origine malgré des éraflures de surface superficielles.",
                "🟡 Une marmite usagée jetée récemment dans le canal, reposant au fond près de la berge."
            ]
        },
        "fun_fact": "La marmite en fonte émaillée combine une structure massive en fonte de fer et un revêtement protecteur d'émail (une couche de verre vitrifié à haute température). Si l'émail protège le fer de la rouille, les chocs mécaniques dans l'eau finissent par le craqueler, exposant la fonte à une corrosion active. L'émail éclaté forme de minuscules morceaux de verre tranchants qui peuvent être ingérés par la faune. Ce déchet métallique lourd doit rejoindre la benne à métaux en décharge."
    },
    {
        "id": 195,
        "descriptions": {
            "badly damaged": [
                "🔴 Une plaque d'immatriculation en aluminium complètement tordue et déchirée en morceaux.",
                "🔴 Ce panneau montre un film plastique réfléchissant entièrement cloqué et décollé en lambeaux délavés.",
                "🔴 Des débris de métal souple d'immatriculation à moitié enfouis sous le limon et le sable grossier."
            ],
            "damaged": [
                "🟠 Une plaque d'immatriculation présentant des enfoncements prononcés et des lettres effacées.",
                "🟠 Cette plaque métallique montre des éraflures profondes dues au frottement répété des galets.",
                "🟠 Une plaque déformée par des courants fluviaux violents, contenant de la vase sur ses bordures."
            ],
            "worn": [
                "🟡 Une plaque d'immatriculation en aluminium entière bien que sale, avec ses numéros encore identifiables.",
                "🟡 Cette plaque a conservé ses rivets de fixation en plastique malgré quelques traces d'usure.",
                "🟡 Une plaque minéralogique usagée jetée récemment, reposant à plat sur un lit de galets clairs."
            ]
        },
        "fun_fact": "La plaque d'immatriculation est fabriquée en aluminium léger recouvert d'un film plastique rétro-réfléchissant adhésif (souvent en polyuréthane ou acrylique). Dans l'eau, le plastique se dégrade sous l'action combinée du soleil et du sel, libérant des microparticules plastiques toxiques. L'aluminium s'oxyde très lentement en formant une fine couche d'alumine protectrice qui stoppe la corrosion. Ce déchet métallique doit être collecté pour être valorisé via la benne à métaux de la décharge."
    },
    {
        "id": 196,
        "descriptions": {
            "badly damaged": [
                "🔴 Un pneu de voiture dont les flancs sont déchirés, révélant la carcasse métallique interne rouillée.",
                "🔴 Ce pneu montre une structure en caoutchouc craquelée et encroûtée de vase noire fétide.",
                "🔴 Des morceaux de bande de roulement de pneu usé, gisant à moitié enterrés sous les sédiments."
            ],
            "damaged": [
                "🟠 Un pneu de voiture présentant des hernies prononcées et des coupures profondes sur ses flancs.",
                "🟠 Ce pneu montre des rainures bouchées par des sédiments argileux et de petites coquilles marines.",
                "🟠 Un pneu déformé par l'écrasement hydrographique de fond, contenant du sable noir grossier."
            ],
            "worn": [
                "🟡 Un pneu de voiture entier bien que sale et usé, reposant droit sur le lit de la rivière.",
                "🟡 Ce pneu a conservé sa forme circulaire caractéristique malgré des éraflures superficielles.",
                "🟡 Un pneu usagé jeté récemment dans le canal, reposant au fond près de la berge de sable."
            ]
        },
        "fun_fact": "Le pneu de voiture usé est un déchet composite complexe fait de caoutchouc vulcanisé (naturel et synthétique), de noir de carbone, de silice, de fils d'acier et de fibres textiles synthétiques. Dans l'eau, le caoutchouc ne se dégrade pas mais subit une érosion lente qui libère des particules de pneu chargées de métaux lourds (zinc, cadmium, plomb) et de composés organiques toxiques (HAP). Ces substances s'infiltrent dans la chaîne trophique aquatique. Il doit être recyclé par des filières spécialisées en décharge."
    },
    {
        "id": 197,
        "descriptions": {
            "badly damaged": [
                "🔴 Une batterie de vélo dont le boîtier en aluminium est fendu, laissant fuir une pâte chimique noire.",
                "🔴 Cet accumulateur montre des fiches de connexion brûlées et des circuits rongés par l'oxydation salée.",
                "🔴 Des débris de cellules de batterie lithium brisées, gisant au fond du canal dans de la vase fétide."
            ],
            "damaged": [
                "🟠 Une batterie de vélo électrique présentant des fissures majeures sur sa coque plastique dur.",
                "🟠 Cet appareil montre une poignée de transport arrachée et des traces de corrosion verte sur ses contacts.",
                "🟠 Une batterie déformée par des chocs violents contre des rochers, contenant du sable et du limon."
            ],
            "worn": [
                "🟡 Une batterie de vélo électrique entière bien que très sale, avec son témoin de charge intact.",
                "🟡 Cet accumulateur a conservé sa structure étanche malgré quelques éraflures superficielles.",
                "🟡 Une batterie de vélo usagée jetée récemment, reposant sur le flanc près de la rive."
            ]
        },
        "fun_fact": "La batterie de vélo électrique est un accumulateur lithium-ion de grande capacité composé de dizaines de cellules cylindriques enveloppées dans un boîtier étanche en plastique ou en aluminium avec un circuit électronique de gestion (BMS). Sa chute dans l'eau représente un danger écologique extrême. L'eau provoque un court-circuit, puis une corrosion qui libère des électrolytes inflammables et des métaux lourds hautement toxiques (lithium, cobalt, nickel). Elle doit être déposée en décharge."
    },
    {
        "id": 198,
        "descriptions": {
            "badly damaged": [
                "🔴 Un extincteur en acier lourd complètement rouillé et percé, dont la lance de projection a disparu.",
                "🔴 Cet appareil de sécurité montre une poignée métallique grippée et un manomètre brisé.",
                "🔴 Une carcasse d'extincteur encroûtée de sédiments marins calcaires durs, d'algues et de coquilles."
            ],
            "damaged": [
                "🟠 Un extincteur métallique présentant de profondes rayures abrasives et des enfoncements majeurs.",
                "🟠 Cet extincteur montre des traces de rouille rousse prononcées sur ses soudures de base.",
                "🟠 Une bouteille d'extincteur déformée par l'écrasement mécanique, remplie de sédiments sableux."
            ],
            "worn": [
                "🟡 Un extincteur vide entier bien que sale, présentant sa goupille de sécurité encore en place.",
                "🟡 Cet extincteur a conservé sa couleur rouge d'origine malgré des traces superficielles d'oxydation.",
                "🟡 Un extincteur usagé jeté récemment dans le canal, reposant au fond près de la berge."
            ]
        },
        "fun_fact": "L'extincteur vide est un récipient sous pression en acier ou en aluminium épais, contenant des résidus d'agents d'extinction (poudre chimique, mousse ou CO2) et de gaz propulseurs. S'il fuit dans l'eau, les résidus chimiques (comme les phosphates d'ammonium des poudres) agissent comme des engrais puissants, favorisant l'eutrophisation de l'eau qui asphyxie les poissons. La bouteille métallique subit une corrosion lente de surface. Il doit être ramené en déchetterie pour traitement."
    },
    {
        "id": 199,
        "descriptions": {
            "badly damaged": [
                "🔴 Un siège auto dont l'armature en plastique est brisée en morceaux coupants et la mousse pourrie.",
                "🔴 Cet équipement montre des tissus déchirés en lambeaux décolorés et gluants de vase noire.",
                "🔴 Des débris de plastique rigide et de boucles de ceinture rouillées, enfouis sous le gravier."
            ],
            "damaged": [
                "🟠 Un siège auto pour bébé présentant des fissures majeures sur sa structure plastique dur.",
                "🟠 Cet équipement montre des sangles de sécurité emmêlées et des moisissures intérieures importantes.",
                "🟠 Un siège auto déformé par l'écrasement mécanique hydrographique, rempli de sédiments fins."
            ],
            "worn": [
                "🟡 Un siège auto pour bébé entier bien que sale et mouillé, flottant à la dérive sur l'eau.",
                "🟡 Cet équipement a conservé sa poignée de transport en plastique malgré des éraflures.",
                "🟡 Un siège auto usagé jeté récemment dans le ruisseau, reposant sur la rive sablonneuse."
            ]
        },
        "fun_fact": "Le siège auto pour bébé combine une coque robuste en polypropylène injecté, un rembourrage en mousse de polyuréthane ou de polystyrène, et des sangles en polyester haute résistance dotées de boucles en acier. En milieu aquatique, la coque et la mousse résistent des siècles à la dégradation biologique, se fragmentant lentement en millions de microparticules de plastique qui empoisonnent les poissons et se déposent sur les sédiments de fond. Il doit être déposé aux encombrants en décharge."
    },
    {
        "id": 200,
        "descriptions": {
            "badly damaged": [
                "🔴 Une trottinette en aluminium brisée au niveau de sa charnière, le guidon tordu en angle droit.",
                "🔴 Cet appareil montre des roues en plastique usées et fendues, couvertes de vase noire.",
                "🔴 Une carcasse métallique de trottinette encroûtée de rouille blanche et de coquilles."
            ],
            "damaged": [
                "🟠 Une trottinette pliable présentant des fissures sur son plateau en acier et des poignées arrachées.",
                "🟠 Cet appareil montre des roulements de roues grippés par la corrosion et des rayures abrasives.",
                "🟠 Une trottinette déformée par des chocs répétés contre les cailloux de fond, couverte de vase."
            ],
            "worn": [
                "🟡 Une trottinette en aluminium entière bien que sale, avec son mécanisme de pliage encore mobile.",
                "🟡 Cette trottinette a conservé son frein arrière métallique malgré des éraflures de surface.",
                "🟡 Une trottinette usagée jetée récemment, reposant sur le flanc au fond de l'eau claire."
            ]
        },
        "fun_fact": "La trottinette pliable cassée est un déchet de loisir principalement fabriqué en aluminium (pour le cadre et le plateau) et en acier (pour la fourche et le frein), avec des roues en polyuréthane. L'aluminium s'oxyde lentement à l'air et à l'eau douce en développant une couche protectrice blanche d'alumine, tandis que l'acier rouille rapidement. Les roues en polyuréthane se dégradent physiquement en libérant des particules de plastique. Ce déchet métallique volumineux doit rejoindre la benne de la décharge."
    },
    {
        "id": 201,
        "descriptions": {
            "badly damaged": [
                "🔴 Un four de cuisine complètement écrasé et rouillé, dont la porte en verre trempé est éclatée en morceaux.",
                "🔴 Cet appareil montre des résistances internes coupées et des isolants en laine de roche pleins de boue.",
                "🔴 Une carcasse métallique de four encroûtée de sédiments marins calcaires durs et de coquilles."
            ],
            "damaged": [
                "🟠 Un four de cuisine présentant des déformations importantes sur ses parois métalliques rouillées.",
                "🟠 Cet appareil de cuisine montre des boutons de réglage cassés et des circuits électriques oxydés.",
                "🟠 Un four déformé par l'écrasement mécanique sous l'eau, contenant du sable et du limon argileux."
            ],
            "worn": [
                "🟡 Un four de cuisine en acier entier bien que sale, avec sa poignée de porte métallique en place.",
                "🟡 Cet appareil a conservé sa grille intérieure malgré des traces d'oxydation superficielles.",
                "🟡 Un four de cuisine usagé jeté récemment dans le canal, reposant au fond près de la berge."
            ]
        },
        "fun_fact": "Le four de cuisine encastrable est un gros équipement électroménager (GEM) composé de tôle d'acier peinte ou inoxydable, de panneaux de laine de roche ou de verre (pour l'isolation thermique), de vitres en verre trempé et de circuits de commande électriques. Son abandon au fond d'un cours d'eau constitue une source majeure de pollution physique et chimique : l'acier se corrode activement, l'isolant thermique se désagrège en fibres irritantes pour les poissons, et les circuits polluent l'eau. Il doit aller en décharge."
    },
    {
        "id": 202,
        "descriptions": {
            "badly damaged": [
                "🔴 Un lave-vaisselle dont la carrosserie en tôle est complètement percée par la rouille et déformée.",
                "🔴 Cet appareil montre une cuve interne en plastique fissurée et remplie de vase noire fétide.",
                "🔴 Une carcasse de lave-vaisselle encroûtée de calcaire et de coquilles, à moitié enfouie sous le sable."
            ],
            "damaged": [
                "🟠 Un lave-vaisselle présentant des déformations sur sa porte et des paniers internes tordus.",
                "🟠 Cet appareil ménager montre des tuyaux d'évacuation en plastique fendus et des circuits oxydés.",
                "🟠 Un lave-vaisselle déformé par l'écrasement mécanique sous des galets, contenant de la vase."
            ],
            "worn": [
                "🟡 Un lave-vaisselle entier bien que sale, avec son tuyau d'alimentation encore attaché au dos.",
                "🟡 Cet appareil a conservé son bouton rotatif de sélection malgré des éraflures superficielles.",
                "🟡 Un lave-vaisselle usagé jeté récemment dans le canal, reposant au fond de l'eau claire."
            ]
        },
        "fun_fact": "Le lave-vaisselle HS fait partie de la catégorie des gros équipements électroménagers (GEM) hors d'usage. Il rassemble de l'acier (carrosserie), du polypropylène (cuve et paniers), du cuivre (moteur de la pompe de cyclage) et divers composants électroniques. Sa présence au fond d'une rivière modifie les écoulements d'eau locaux et crée des zones d'envasement. De plus, la corrosion du moteur et des circuits libère des ions métalliques toxiques pour les branchies des poissons. Il doit rejoindre la décharge."
    }
]

updates_en = [
    {
        "id": 193,
        "descriptions": {
            "badly damaged": [
                "🔴 A cast iron cooking plate completely broken into several heavy and rusted blocks.",
                "🔴 This kitchen element shows a cast iron surface crumbling as iron oxide scales.",
                "🔴 A massive metal plate half-buried under pebbles and black clayey silt."
            ],
            "damaged": [
                "🟠 A cooking plate showing deep cracks running through its metallic structure.",
                "🟠 This object shows significant white lime deposits and torn control knobs.",
                "🟠 A cast iron plate deformed by the weight of other bottom debris, covered in silt."
            ],
            "worn": [
                "🟡 A cast iron cooking plate whole although dirty, resting flat on the bottom.",
                "🟡 This cast iron element has kept its rectangular appearance despite superficial scratches.",
                "🟡 A used cooking plate discarded recently, encrusted with a light veil of silt."
            ]
        },
        "fun_fact": "The cast iron cooking plate is very heavy kitchen equipment made of gray cast iron. Abandoned in water, it presents a physical risk of crushing benthos living on bottom sediments. Although the iron oxidizes slowly to form harmless rust, its cracks or fractured edges can injure fish. In addition, cast iron takes many decades to fully corrode. It must be deposited in the metal scrap bin at the landfill."
    },
    {
        "id": 194,
        "descriptions": {
            "badly damaged": [
                "🔴 A cast iron pot whose enamel is completely peeled and the bottom pierced by corrosion.",
                "🔴 This heavy container shows a broken metal handle and a structure encrusted with foul black silt.",
                "🔴 Enameled cast iron debris broken into sharp shards and embedded with lime and shells."
            ],
            "damaged": [
                "🟠 A cast iron pot showing large chips on its colored enamel coating.",
                "🟠 This container shows pronounced brown rust marks on its sides and a bent handle.",
                "🟠 A pot deformed by violent impacts against rocks, containing fine sand."
            ],
            "worn": [
                "🟡 An enameled cast iron pot whole although dirty, with its two handles still in place.",
                "🟡 This container has kept its original color despite superficial surface scratches.",
                "🟡 A used pot discarded recently in the canal, resting at the bottom near the bank."
            ]
        },
        "fun_fact": "The enameled cast iron pot combines a massive cast iron structure and a protective enamel coating (a glass layer vitrified at high temperature). While enamel protects the iron from rust, mechanical shocks in water eventually crack it, exposing the cast iron to active corrosion. The chipped enamel forms tiny sharp glass pieces that can be ingested by fauna, causing digestive lesions. It must go to the landfill."
    },
    {
        "id": 195,
        "descriptions": {
            "badly damaged": [
                "🔴 An aluminum license plate completely bent and torn into pieces.",
                "🔴 This sign shows a reflective plastic film completely blistered and peeling off in faded shreds.",
                "🔴 Flexible license plate metal debris half-buried under silt and coarse sand."
            ],
            "damaged": [
                "🟠 A license plate showing pronounced dents and erased letters.",
                "🟠 This metal plate shows deep scratches due to repeated pebble friction.",
                "🟠 A plate deformed by violent river currents, containing silt on its edges."
            ],
            "worn": [
                "🟡 A whole aluminum license plate although dirty, with its numbers still identifiable.",
                "🟡 This plate has kept its plastic attachment rivets despite some signs of wear.",
                "🟡 A used license plate discarded recently, resting flat on a bed of light pebbles."
            ]
        },
        "fun_fact": "The license plate is made of lightweight aluminum covered with a retro-reflective adhesive plastic film (often polyurethane or acrylic). In water, the plastic degrades under the combined action of sun and salt, releasing toxic microplastic particles. The aluminum oxidizes very slowly, forming a thin protective layer of alumina that stops corrosion. This metallic waste must be collected in the scrap metal bin at the landfill."
    },
    {
        "id": 196,
        "descriptions": {
            "badly damaged": [
                "🔴 A car tire whose sidewalls are torn, revealing the rusted internal steel cords.",
                "🔴 This tire shows a cracked rubber structure encrusted with foul black silt.",
                "🔴 Tread pieces from a worn car tire, lying half-buried under bottom sediments."
            ],
            "damaged": [
                "🟠 A car tire showing pronounced bulges and deep cuts on its sidewalls.",
                "🟠 This tire shows grooves clogged with clayey sediments and small marine shells.",
                "🟠 A tire deformed by hydrographic bottom crushing, containing coarse black sand."
            ],
            "worn": [
                "🟡 A car tire whole although dirty and worn, resting straight on the riverbed.",
                "🟡 This tire has kept its characteristic circular shape despite superficial scratches.",
                "🟡 A used tire discarded recently in the canal, resting at the bottom near the sandbank."
            ]
        },
        "fun_fact": "The worn car tire is complex composite waste made of vulcanized rubber (natural and synthetic), carbon black, silica, steel wire cords, and synthetic textile fibers. In water, the rubber does not degrade but undergoes slow erosion that releases tire wear particles loaded with heavy metals (zinc, cadmium, lead) and toxic organic compounds (PAHs). These substances seep into the aquatic food chain. It must be recycled at the landfill."
    },
    {
        "id": 197,
        "descriptions": {
            "badly damaged": [
                "🔴 A bicycle battery whose aluminum casing is split, leaking a black chemical paste.",
                "🔴 This accumulator shows burnt connection plugs and circuits eaten away by salty oxidation.",
                "🔴 Broken lithium battery cell debris, lying at the bottom of the canal in foul silt."
            ],
            "damaged": [
                "🟠 An electric bicycle battery showing major cracks on its hard plastic shell.",
                "🟠 This device shows a torn carrying handle and green corrosion marks on its contacts.",
                "🟠 A battery deformed by violent impacts against rocks, containing sand and silt."
            ],
            "worn": [
                "🟡 A whole electric bicycle battery although very dirty, with its charge indicator intact.",
                "🟡 This accumulator has kept its waterproof structure despite some superficial scratches.",
                "🟡 A used bicycle battery discarded recently, resting on its side near the bank."
            ]
        },
        "fun_fact": "The electric bike battery is a large capacity lithium-ion pack composed of dozens of cylindrical cells wrapped in a waterproof plastic or aluminum casing with an electronic battery management system (BMS). Its fall into water represents an extreme ecological danger. Water causes a short circuit, then corrosion that releases flammable electrolytes and highly toxic heavy metals (lithium, cobalt, nickel). It must be disposed of in the landfill."
    },
    {
        "id": 198,
        "descriptions": {
            "badly damaged": [
                "🔴 A heavy steel fire extinguisher completely rusted and pierced, its discharge hose gone.",
                "🔴 This safety device shows a seized metal handle and a broken pressure gauge.",
                "🔴 A fire extinguisher casing encrusted with hard calcareous marine sediments, algae, and shells."
            ],
            "damaged": [
                "🟠 A metal fire extinguisher showing deep abrasive scratches and major dents.",
                "🟠 This extinguisher shows pronounced red rust marks on its base welds.",
                "🟠 An extinguisher cylinder deformed by mechanical crushing, filled with sandy sediments."
            ],
            "worn": [
                "🟡 An empty fire extinguisher whole although dirty, showing its safety pin still in place.",
                "🟡 This extinguisher has kept its original red color despite superficial signs of oxidation.",
                "🟡 A used fire extinguisher discarded recently in the canal, resting at the bottom near the bank."
            ]
        },
        "fun_fact": "The empty fire extinguisher is a pressurized container made of steel or thick aluminum, containing residues of extinguishing agents (dry chemical, foam, or CO2) and propellant gases. If it leaks in water, the chemical residues (such as ammonium phosphates in powders) act as powerful fertilizers, promoting water eutrophication that suffocates fish. The metal cylinder undergoes slow surface corrosion. It must be returned to the landfill."
    },
    {
        "id": 199,
        "descriptions": {
            "badly damaged": [
                "🔴 A car seat whose plastic frame is broken into sharp pieces and padding rotted.",
                "🔴 This equipment shows fabrics torn into faded shreds, slimy with black silt.",
                "🔴 Rigid plastic and rusted belt buckle debris, buried under the gravel."
            ],
            "damaged": [
                "🟠 A baby car seat showing major cracks on its hard plastic structure.",
                "🟠 This equipment shows tangled safety straps and significant internal mold.",
                "🟠 A car seat deformed by hydrographic mechanical crushing, filled with fine sediments."
            ],
            "worn": [
                "🟡 A baby car seat whole although dirty and wet, floating adrift on the water.",
                "🟡 This equipment has kept its plastic carrying handle despite minor scratches.",
                "🟡 A used car seat discarded recently in the stream, resting on the sandy bank."
            ]
        },
        "fun_fact": "The baby car seat combines a robust injected polypropylene shell, polyurethane or polystyrene foam padding, and high-strength polyester straps with steel buckles. In the aquatic environment, the shell and foam resist biological degradation for centuries, slowly fragmenting into millions of plastic microparticles that poison fish and settle on bottom sediments. It must be deposited at the landfill."
    },
    {
        "id": 200,
        "descriptions": {
            "badly damaged": [
                "🔴 An aluminum scooter broken at its hinge, the handlebar bent at a right angle.",
                "🔴 This device shows worn and split plastic wheels, covered in black silt.",
                "🔴 A metallic scooter casing encrusted with white rust and shells."
            ],
            "damaged": [
                "🟠 A folding scooter showing cracks on its steel deck and torn handles.",
                "🟠 This device shows wheel bearings seized by corrosion and abrasive scratches.",
                "🟠 A scooter deformed by repeated impacts against bottom stones, covered in silt."
            ],
            "worn": [
                "🟡 An aluminum scooter whole although dirty, with its folding mechanism still movable.",
                "🟡 This scooter has kept its metal rear brake despite surface scratches.",
                "🟡 A used scooter discarded recently, resting on its side at the bottom of the clear water."
            ]
        },
        "fun_fact": "The broken folding scooter is recreational waste mainly made of aluminum (for the frame and deck) and steel (for the fork and brake), with polyurethane wheels. Aluminum oxidizes slowly in air and fresh water, developing a white protective layer of alumina, while steel rusts quickly. The polyurethane wheels degrade physically, releasing plastic particles. This bulky metallic waste must be sent to the landfill."
    },
    {
        "id": 201,
        "descriptions": {
            "badly damaged": [
                "🔴 A kitchen oven completely crushed and rusted, its tempered glass door shattered into pieces.",
                "🔴 This appliance shows internal heating elements cut and rock wool insulation full of mud.",
                "🔴 A metal oven casing encrusted with hard calcareous marine sediments and shells."
            ],
            "damaged": [
                "🟠 A kitchen oven showing significant deformations on its rusted metal walls.",
                "🟠 This kitchen appliance shows broken control knobs and oxidized electrical circuits.",
                "🟠 An oven deformed by mechanical crushing under water, containing sand and clayey silt."
            ],
            "worn": [
                "🟡 A kitchen oven made of steel whole although dirty, with its metal door handle in place.",
                "🟡 This appliance has kept its internal rack despite superficial signs of oxidation.",
                "🟡 A used kitchen oven discarded recently in the canal, resting at the bottom near the bank."
            ]
        },
        "fun_fact": "The built-in kitchen oven is a large household appliance (LHA) made of painted or stainless steel sheet, rock wool or glass wool panels (for thermal insulation), tempered glass panes, and electrical control circuits. Its abandonment at the bottom of a waterway constitutes a major source of physical and chemical pollution: the steel actively corrodes, the insulation disintegrates into fibers irritating to fish, and circuits pollute the water. It must go to the landfill."
    },
    {
        "id": 202,
        "descriptions": {
            "badly damaged": [
                "🔴 A dishwasher whose sheet metal body is completely pierced by rust and deformed.",
                "🔴 This appliance shows an internal plastic tub cracked and filled with foul black silt.",
                "🔴 A dishwasher casing encrusted with lime and shells, half-buried under the sand."
            ],
            "damaged": [
                "🟠 A dishwasher showing deformations on its door and bent internal racks.",
                "🟠 This household appliance shows split plastic drain hoses and oxidized circuits.",
                "🟠 A dishwasher deformed by mechanical crushing under pebbles, containing silt."
            ],
            "worn": [
                "🟡 A dishwasher whole although dirty, with its inlet hose still attached to the back.",
                "🟡 This appliance has kept its rotary selector knob despite superficial scratches.",
                "🟡 A used dishwasher discarded recently in the canal, resting at the bottom of the clear water."
            ]
        },
        "fun_fact": "The broken dishwasher belongs to the category of large household appliances (LHA) out of use. It gathers steel (body), polypropylene (tub and racks), copper (circulation pump motor), and various electronic components. Its presence at the bottom of a river alters local water flows and creates silting zones. Furthermore, motor and circuit corrosion releases metal ions toxic to fish gills. It must join the landfill."
    }
]

apply_updates(updates_fr, updates_en)
print("Batch 6 updated successfully!")
