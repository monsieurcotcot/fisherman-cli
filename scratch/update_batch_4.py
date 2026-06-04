# -*- coding: utf-8 -*-
from update_junk_util import apply_updates

updates_fr = [
    {
        "id": 173,
        "descriptions": {
            "badly damaged": [
                "🔴 Un bocal en verre cassé en plusieurs éclats coupants et anguleux, à moitié ensevelis dans le gravier.",
                "🔴 Ce récipient transparent montre des cassures nettes et une base encroûtée de sédiments marins durs.",
                "🔴 Des débris de verre épais et poli par le frottement du sable, gisant au fond du cours d'eau."
            ],
            "damaged": [
                "🟠 Un bocal en verre présentant une fêlure importante sur le flanc et un col ébréché.",
                "🟠 Cet objet en verre montre des parois ternies par le calcaire et des traces d'algues brunes à l'intérieur.",
                "🟠 Un contenant en verre déformé par l'érosion du sable, rempli de sédiments argileux sombres."
            ],
            "worn": [
                "🟡 Un bocal en verre transparent entier bien que sale, reposant droit sur le lit de la rivière.",
                "🟡 Ce récipient a conservé sa forme cylindrique malgré des rayures superficielles sur ses parois.",
                "🟡 Un bocal de conserve usagé jeté récemment, dont l'étiquette en papier a été dissoute."
            ]
        },
        "fun_fact": "Le bocal en verre sans couvercle est composé de verre silico-sodo-calcique, un matériau inerte chimiquement mais d'une immense longévité physique (estimée à plus de 4000 ans). S'il ne libère pas de substances chimiques toxiques dans l'eau, sa présence physique présente des risques majeurs. Les éclats de verre cassé peuvent blesser gravement les poissons, les mammifères aquatiques et les usagers de l'eau. De plus, les bocaux entiers peuvent piéger de petits poissons qui y entrent et ne parviennent plus à en sortir. Il doit être trié dans la poubelle verte."
    },
    {
        "id": 174,
        "descriptions": {
            "badly damaged": [
                "🔴 Un flacon de parfum en verre épais brisé en éclats coupants, le vaporisateur en plastique fondu ou brisé.",
                "🔴 Cette bouteille de parfum montre un col en laiton oxydé de vert-de-gris et une base encroûtée de limon noir.",
                "🔴 Des restes de verre coloré poli et de plastique d'un atomiseur de parfum cassé, enfouis dans la vase."
            ],
            "damaged": [
                "🟠 Un flacon de parfum présentant des rayures profondes et un col métallique tordu par les vagues.",
                "🟠 Cet objet en verre montre un bouchon en plastique fêlé et des parois opacifiées par le calcaire.",
                "🟠 Un flacon décoratif fendu sur le côté, contenant de la vase fine et des débris organiques."
            ],
            "worn": [
                "🟡 Un flacon de parfum en verre entier bien que sale, avec sa pompe de vaporisation intacte.",
                "🟡 Ce flacon a conservé ses motifs gravés visibles malgré des éraflures superficielles sur ses faces.",
                "🟡 Un flacon de parfum usagé jeté récemment, reposant sur le flanc au fond du ruisseau."
            ]
        },
        "fun_fact": "Le flacon de parfum vide associe un corps en verre épais (souvent haut de gamme), une pompe de vaporisation mécanique en laiton ou en plastique, et parfois un bouchon multimatière. Bien que le verre soit recyclable à 100% et à l'infini, sa dégradation physique dans la nature prend des milliers d'années. Les résidus de parfum concentrés (huiles essentielles, solvants alcooliques) peuvent perturber chimiquement la faune aquatique locale s'ils se déversent en quantité. Il doit être trié dans la poubelle verte en verre."
    },
    {
        "id": 175,
        "descriptions": {
            "badly damaged": [
                "🔴 Une canette métallique complètement écrasée et rouillée, dont les parois en acier s'effritent au toucher.",
                "🔴 Cet emballage en tôle d'acier montre des déchirures coupantes et une déformation totale remplie de limon.",
                "🔴 Des fragments d'acier de canette rongés par la corrosion saline, à moitié enterrés sous le sable."
            ],
            "damaged": [
                "🟠 Une canette de soda présentant des enfoncements importants et des taches de rouille rousse sur ses flancs.",
                "🟠 Cet emballage métallique montre des rayures abrasives qui ont effacé les motifs imprimés colorés.",
                "🟠 Une canette métallique déformée par le courant, contenant du sable et des dépôts de vase fine."
            ],
            "worn": [
                "🟡 Une canette de soda en acier entière bien que vide et sale, flottant à la surface de l'eau.",
                "🟡 Cet emballage a conservé sa languette d'ouverture en place malgré des éraflures superficielles.",
                "🟡 Une canette de soda usagée jetée récemment, reposant sur un lit de galets de fond."
            ]
        },
        "fun_fact": "La canette de soda en acier (souvent du fer blanc) subit une oxydation rapide en milieu humide. Contrairement à l'aluminium, l'acier se corrode activement jusqu'à sa disparition complète en quelques décennies, libérant des oxydes de fer qui colorent le sédiment en orange. Cependant, tant qu'elle n'est pas désintégrée, les bords tranchants de son ouverture et de ses déchirures constituent des pièges coupants pour les poissons et la faune benthique. Elle doit impérativement être jetée dans la poubelle jaune."
    },
    {
        "id": 176,
        "descriptions": {
            "badly damaged": [
                "🔴 Une brique en carton complètement délaminée, dont les couches de plastique et d'aluminium se détachent.",
                "🔴 Cet emballage montre des parois en carton ramollies et pourries, couvertes de moisissures noires.",
                "🔴 Des débris de carton bouilli et de film d'aluminium d'une brique déchirée, emmêlés dans des algues."
            ],
            "damaged": [
                "🟠 Une brique de lait présentant des déformations importantes et un goulot en plastique fendu.",
                "🟠 Cet emballage cartonné montre des parois gonflées d'eau et une décoloration de ses décors.",
                "🟠 Une brique en carton froissée sur les côtés, contenant de la vase fine et des dépôts de sable."
            ],
            "worn": [
                "🟡 Une brique de lait en carton entière bien que mouillée, flottant à plat à la surface de l'eau.",
                "🟡 Cet emballage a conservé son bouchon à vis en plastique malgré quelques rayures superficielles.",
                "🟡 Une brique cartonnée usagée jetée récemment, reposant près de la berge du canal."
            ]
        },
        "fun_fact": "La brique de lait en carton est un emballage complexe multicouche composé d'environ 75% de carton (pour la rigidité), 20% de polyéthylène plastique (pour l'étanchéité) et 5% d'aluminium (pour la conservation). Cette association intime rend sa décomposition longue : si le carton pourrit en quelques mois, le plastique et l'aluminium persistent plus de 100 ans. Les résidus de lait fermenté polluent également l'eau. Elle doit être triée dans la poubelle jaune pour que ses couches soient séparées et recyclées."
    },
    {
        "id": 177,
        "descriptions": {
            "badly damaged": [
                "🔴 Une bouteille en PEHD complètement aplatie et trouée, dont le plastique jauni s'effrite par morceaux.",
                "🔴 Ce flacon de lait montre de profondes fêlures longitudinales et une surface encroûtée d'algues sèches.",
                "🔴 Des morceaux de plastique blanc rigide provenant d'une bouteille brisée, enfouis dans la boue."
            ],
            "damaged": [
                "🟠 Une bouteille de lait en plastique présentant des bosses prononcées et des rayures abrasives profondes.",
                "🟠 Ce flacon blanc opaque montre une poignée déformée et un goulot encrassé par des dépôts de sédiments.",
                "🟠 Une bouteille en PEHD déformée par le ressac des vagues, contenant du sable grossier."
            ],
            "worn": [
                "🟡 Une bouteille de lait en plastique PEHD blanc opaque entière, flottant à la surface de la rivière.",
                "🟡 Ce flacon a conservé son bouchon en plastique de couleur malgré quelques traces d'usure.",
                "🟡 Une bouteille de lait usagée jetée récemment, reposant sur le flanc au bord de l'eau."
            ]
        },
        "fun_fact": "La bouteille de lait en PEHD (polyéthylène haute densité) est un emballage plastique rigide et opaque. Le PEHD est un matériau plastique très stable chimiquement qui ne se biodégrade pas. Sous l'influence des rayons ultraviolets solaires et du frottement mécanique des galets, la bouteille se fragmente en microparticules de plastique qui adsorbent les polluants chimiques présents dans l'eau. Ces particules pénètrent dans l'organisme des poissons. Elle doit être jetée dans la poubelle jaune pour être recyclée."
    },
    {
        "id": 178,
        "descriptions": {
            "badly damaged": [
                "🔴 Une bombe aérosol en métal complètement écrasée et rouillée, dont le diffuseur en plastique a fondu.",
                "🔴 Cet emballage sous pression montre des perforations par la corrosion et des parois qui s'effritent.",
                "🔴 Une carcasse de bouteille aérosol encroûtée de sédiments calcaires et de vase noire de fond."
            ],
            "damaged": [
                "🟠 Une bombe aérosol présentant des déformations importantes et une oxydation rousse sur ses soudures.",
                "🟠 Cet emballage métallique montre des rayures profondes qui ont effacé les inscriptions de sécurité.",
                "🟠 Une bouteille aérosol déformée par l'écrasement mécanique, remplie de sédiments sableux lourds."
            ],
            "worn": [
                "🟡 Une bombe aérosol vide entière bien que sale, présentant des traces de rouille superficielles.",
                "🟡 Cet emballage a conservé sa valve métallique supérieure malgré des éraflures de surface.",
                "🟡 Une bombe aérosol usagée jetée récemment, reposant sur un lit de galets de rivière."
            ]
        },
        "fun_fact": "La bombe aérosol vide est un emballage métallique en acier ou en aluminium conçu pour contenir des produits sous pression. Même vide, elle présente des risques d'explosion si elle est soumise à de fortes chaleurs. Dans l'eau, sa corrosion libère des oxydes métalliques et de petits résidus de gaz propulseurs ou de produits chimiques actifs (solvants, parfums, agents de charge). Ces produits chimiques contaminent la faune locale. Elle doit être triée dans la poubelle jaune de tri des métaux."
    },
    {
        "id": 179,
        "descriptions": {
            "badly damaged": [
                "🔴 Une boîte de sardines en conserve métallique complètement aplatie et rouillée, aux bords déchiquetés.",
                "🔴 Ce récipient métallique s'effrite en plaques d'oxyde de fer épaisses sous l'action de l'eau.",
                "🔴 Une carcasse d'emballage métallique encroûtée de sédiments calcaires durs, d'algues et de coquilles."
            ],
            "damaged": [
                "🟠 Une boîte de sardines présentant des déformations majeures et un couvercle métallique à demi arraché.",
                "🟠 Cet emballage montre des traces de rouille rousse prononcées et des arêtes très coupantes.",
                "🟠 Une boîte métallique déformée par des chocs répétés, contenant une vase noire huileuse."
            ],
            "worn": [
                "🟡 Une boîte de sardines en acier entière bien que sale, présentant des traces de rouille superficielles.",
                "🟡 Cet emballage a conservé sa forme rectangulaire caractéristique malgré quelques éraflures.",
                "🟡 Une boîte de sardines usagée jetée récemment, reposant sur un lit de sable au fond."
            ]
        },
        "fun_fact": "La boîte de sardines en conserve est un emballage métallique en fer blanc ou en aluminium. Si elle contient encore de l'huile, son immersion provoque la formation d'un film gras en surface qui bloque les échanges d'oxygène. De plus, sa structure en métal ouverte et coupante constitue un réel danger physique pour les animaux marins et d'eau douce, qui peuvent s'y couper ou y rester piégés. Elle doit être vidée de son huile résiduelle puis jetée dans la poubelle de tri jaune."
    },
    {
        "id": 180,
        "descriptions": {
            "badly damaged": [
                "🔴 Une gourde en aluminium complètement écrasée et trouée, dont la peinture extérieure s'est écaillée.",
                "🔴 Ce récipient montre un goulot métallique déformé et un bouchon en plastique cassé et plein de vase.",
                "🔴 Des débris métalliques d'une gourde détruite, à moitié enfouis sous le gravier et le limon de fond."
            ],
            "damaged": [
                "🟠 Une gourde en aluminium présentant des enfoncements prononcés et des rayures abrasives profondes.",
                "🟠 Ce récipient métallique montre une perte de couleur extérieure due aux frottements des galets.",
                "🟠 Une gourde de sport déformée par des chocs violents, contenant de la vase fine et du sable."
            ],
            "worn": [
                "🟡 Une gourde en aluminium entière bien que sale et présentant des éraflures superficielles.",
                "🟡 Ce récipient métallique a conservé sa sangle de transport malgré des traces d'usure.",
                "🟡 Une gourde de sport usagée jetée récemment, reposant sur le flanc près de la berge."
            ]
        },
        "fun_fact": "La gourde en aluminium cabossée est fabriquée en aluminium léger et durable, souvent recouverte à l'intérieur d'un vernis en résine époxy pour éviter le contact direct du métal avec les boissons. Bien que l'aluminium soit hautement recyclable, son abandon dans la nature est préjudiciable. L'aluminium s'oxyde lentement en formant de l'alumine protectrice, mais le vernis intérieur peut s'effriter sous l'effet de l'érosion physique et libérer des composés organiques nocifs. Elle doit être triée dans la poubelle jaune."
    },
    {
        "id": 181,
        "descriptions": {
            "badly damaged": [
                "🔴 Une souris d'ordinateur dont la coque plastique est brisée en morceaux et le capteur optique oxydé.",
                "🔴 Cet appareil montre des circuits imprimés verdis par la corrosion et des boutons absents ou bloqués.",
                "🔴 Des débris de plastique et de composants électroniques de souris brisée, encroûtés de boue noire."
            ],
            "damaged": [
                "🟠 Une souris d'ordinateur présentant des fissures majeures sur sa coque en plastique et des rayures.",
                "🟠 Cet objet montre une molette de défilement bloquée par des dépôts argileux et du limon.",
                "🟠 Une souris sans fil déformée par un choc mécanique violent, contenant du sable sous son capot."
            ],
            "worn": [
                "🟡 Une souris d'ordinateur sans fil entière bien que sale, reposant près de la rive.",
                "🟡 Cet appareil a conservé sa coque en plastique intacte malgré de légères traces d'oxydation.",
                "🟡 Une souris d'ordinateur usagée jetée récemment, reposant au fond de l'eau claire."
            ]
        },
        "fun_fact": "La souris d'ordinateur sans fil est un déchet d'équipements électriques et électroniques (DEEE). Elle contient une coque en ABS rigide, un capteur optique à LED ou laser, des micro-interrupteurs, un circuit imprimé riche en métaux et un compartiment à piles. En cas d'immersion prolongée, l'infiltration d'eau provoque des réactions d'oxydation complexes qui libèrent des substances toxiques pour la vie aquatique (plomb, cadmium des soudures). Elle doit rejoindre le bac de tri des DEEE de la poubelle grise."
    },
    {
        "id": 182,
        "descriptions": {
            "badly damaged": [
                "🔴 Un boîtier de chargeur électrique dont la coque en plastique s'est ouverte, révélant le transformateur rouillé.",
                "🔴 Ce chargeur montre des fiches de connexion en laiton complètement tordues et oxydées vertes.",
                "🔴 Des restes de plastique brûlé et de fils électriques de chargeur détruit, enfouis dans le limon."
            ],
            "damaged": [
                "🟠 Un chargeur de téléphone présentant des fissures sur sa coque plastique dure et des broches tordues.",
                "🟠 Cet appareil montre des traces de corrosion verte sur ses composants métalliques humides.",
                "🟠 Un chargeur déformé par l'écrasement mécanique sous des galets, contenant de la vase fine."
            ],
            "worn": [
                "🟡 Un chargeur de téléphone portable entier bien que sale, avec sa coque plastique sans fissure.",
                "🟡 Cet adaptateur électrique a conservé ses fiches en laiton utilisables malgré des éraflures.",
                "🟡 Un chargeur usagé jeté récemment dans le canal, reposant au fond de l'eau."
            ]
        },
        "fun_fact": "Le chargeur de téléphone portable est un adaptateur secteur électronique composé d'un boîtier en plastique ABS autoextinguible, d'un transformateur de tension (bobinage de cuivre et noyau de fer) et de composants semi-conducteurs montés sur circuit imprimé. Sa chute au fond d'un cours d'eau expose le milieu aquatique à des fuites de métaux lourds et d'additifs plastiques ignifugés (retardateurs de flamme bromés) très toxiques et persistants. Il doit être déposé dans le bac de tri des DEEE de la poubelle grise."
    }
]

updates_en = [
    {
        "id": 173,
        "descriptions": {
            "badly damaged": [
                "🔴 A glass jar broken into several sharp and angular shards, half-buried in the gravel.",
                "🔴 This transparent container shows clean breaks and a base encrusted with hard marine sediments.",
                "🔴 Thick glass debris polished by sand friction, lying at the bottom of the waterway."
            ],
            "damaged": [
                "🟠 A glass jar showing a significant crack on the side and a chipped neck.",
                "🟠 This glass object shows walls tarnished by lime and traces of brown algae inside.",
                "🟠 A glass container deformed by sand erosion, filled with dark clayey sediments."
            ],
            "worn": [
                "🟡 A whole transparent glass jar although dirty, resting straight on the riverbed.",
                "🟡 This container has kept its cylindrical shape despite superficial scratches on its walls.",
                "🟡 A used canning jar discarded recently, its paper label dissolved."
            ]
        },
        "fun_fact": "The glass jar without a lid is composed of soda-lime-silica glass, a material that is chemically inert but has immense physical longevity (estimated at over 4000 years). While it does not release toxic chemicals into the water, its physical presence presents major risks. Shards of broken glass can severely injure fish, aquatic mammals, and human users. In addition, whole jars can trap small fish that enter and cannot find their way out. It must be sorted in the green glass bin."
    },
    {
        "id": 174,
        "descriptions": {
            "badly damaged": [
                "🔴 A thick glass perfume bottle broken into sharp shards, the plastic spray mechanism melted or broken.",
                "🔴 This perfume bottle shows an oxidized brass neck covered in verdigris and a base encrusted with black silt.",
                "🔴 Remains of polished colored glass and plastic from a broken perfume atomizer, buried in the mud."
            ],
            "damaged": [
                "🟠 A perfume bottle showing deep scratches and a metal neck twisted by the waves.",
                "🟠 This glass object shows a cracked plastic cap and walls opacified by lime.",
                "🟠 A decorative bottle split on the side, containing fine silt and organic debris."
            ],
            "worn": [
                "🟡 A whole glass perfume bottle although dirty, with its spray pump intact.",
                "🟡 This bottle has kept its engraved patterns visible despite superficial scratches on its faces.",
                "🟡 A used perfume bottle discarded recently, resting on its side at the bottom of the stream."
            ]
        },
        "fun_fact": "The empty perfume bottle combines a thick glass body (often high-end), a mechanical spray pump made of brass or plastic, and sometimes a multi-material cap. Although glass is 100% recyclable and infinitely so, its physical breakdown in nature takes thousands of years. Concentrated perfume residues (essential oils, alcoholic solvents) can chemically disrupt the local aquatic fauna if released in quantity. It must be sorted in the green glass bin."
    },
    {
        "id": 175,
        "descriptions": {
            "badly damaged": [
                "🔴 A metal can completely crushed and rusted, its steel walls crumbling to the touch.",
                "🔴 This steel sheet packaging shows sharp tears and a total deformation filled with silt.",
                "🔴 Steel fragments from a can eroded by saline corrosion, half-buried under the sand."
            ],
            "damaged": [
                "🟠 A soda can showing significant dents and red rust spots on its sides.",
                "🟠 This metal packaging shows abrasive scratches that have erased the colorful printed patterns.",
                "🟠 A metal can deformed by the current, containing sand and fine silt deposits."
            ],
            "worn": [
                "🟡 A whole steel soda can although empty and dirty, floating on the water surface.",
                "🟡 This packaging has kept its pull tab in place despite superficial scratches.",
                "🟡 A used soda can discarded recently, resting on a bed of bottom pebbles."
            ]
        },
        "fun_fact": "The steel soda can (often tinplate) undergoes rapid oxidation in a humid environment. Unlike aluminum, steel actively corrodes until its complete disappearance in a few decades, releasing iron oxides that color the sediment orange. However, as long as it is not disintegrated, the sharp edges of its opening and tears constitute cutting traps for fish and benthic fauna. It must be thrown into the yellow recycling bin."
    },
    {
        "id": 176,
        "descriptions": {
            "badly damaged": [
                "🔴 A cardboard brick completely delaminated, its plastic and aluminum layers peeling off.",
                "🔴 This package shows softened and rotted cardboard walls, covered with black mold.",
                "🔴 Debris of boiled cardboard and aluminum film from a torn brick, tangled in algae."
            ],
            "damaged": [
                "🟠 A milk carton showing significant deformations and a cracked plastic spout.",
                "🟠 This cardboard package shows walls swollen with water and discoloration of its designs.",
                "🟠 A cardboard brick crumpled on the sides, containing fine silt and sand deposits."
            ],
            "worn": [
                "🟡 A whole cardboard milk brick although wet, floating flat on the water surface.",
                "🟡 This package has kept its plastic screw cap despite some superficial scratches.",
                "🟡 A used cardboard brick discarded recently, resting near the bank of the canal."
            ]
        },
        "fun_fact": "The cardboard milk brick is a complex multi-layer package composed of approximately 75% cardboard (for rigidity), 20% plastic polyethylene (for waterproofing), and 5% aluminum (for preservation). This intimate combination makes its decomposition slow: while the cardboard rots in a few months, the plastic and aluminum persist for over 100 years. Residues of fermented milk also pollute the water. It must be sorted in the yellow bin to be recycled."
    },
    {
        "id": 177,
        "descriptions": {
            "badly damaged": [
                "🔴 An HDPE bottle completely flattened and holed, its yellowed plastic crumbling in pieces.",
                "🔴 This milk flask shows deep longitudinal cracks and a surface encrusted with dry algae.",
                "🔴 White rigid plastic pieces from a broken bottle, buried in the mud."
            ],
            "damaged": [
                "🟠 A plastic milk bottle showing pronounced dents and deep abrasive scratches.",
                "🟠 This opaque white flask shows a deformed handle and a neck clogged with sediment deposits.",
                "🟠 An HDPE bottle deformed by the backwash of the waves, containing coarse sand."
            ],
            "worn": [
                "🟡 An opaque white HDPE plastic milk bottle whole, floating on the surface of the river.",
                "🟡 This flask has kept its colored plastic cap despite some signs of wear.",
                "🟡 A used milk bottle discarded recently, resting on its side at the edge of the water."
            ]
        },
        "fun_fact": "The HDPE (high-density polyethylene) milk bottle is a rigid and opaque plastic package. HDPE is a very chemically stable plastic material that does not biodegrade. Under the influence of solar ultraviolet rays and the mechanical friction of pebbles, the bottle fragments into plastic microparticles that adsorb chemical pollutants present in the water. These particles enter the bodies of fish. It must be thrown in the yellow recycling bin."
    },
    {
        "id": 178,
        "descriptions": {
            "badly damaged": [
                "🔴 A metal aerosol can completely crushed and rusted, its plastic diffuser melted.",
                "🔴 This pressurized package shows corrosion perforations and walls that crumble.",
                "🔴 An aerosol bottle casing encrusted with calcareous sediments and black bottom silt."
            ],
            "damaged": [
                "🟠 An aerosol can showing significant deformations and red oxidation on its welds.",
                "🟠 This metal package shows deep scratches that have erased the safety labels.",
                "🟠 An aerosol bottle deformed by mechanical crushing, filled with heavy sandy sediments."
            ],
            "worn": [
                "🟡 An empty aerosol can whole although dirty, showing superficial rust marks.",
                "🟡 This package has kept its upper metal valve despite surface scratches.",
                "🟡 A used aerosol can discarded recently, resting on a bed of river pebbles."
            ]
        },
        "fun_fact": "The empty aerosol can is metal packaging made of steel or aluminum designed to contain products under pressure. Even empty, it presents risks of explosion if subjected to high heat. In water, its corrosion releases metal oxides and small residues of propellant gases or active chemicals (solvents, fragrances, bulking agents). These chemicals contaminate local fauna. It must be sorted in the yellow recycling bin for metals."
    },
    {
        "id": 179,
        "descriptions": {
            "badly damaged": [
                "🔴 A metal sardine can completely flattened and rusted, with shredded edges.",
                "🔴 This metal container crumbles into thick iron oxide plates under the action of water.",
                "🔴 A metal packaging casing encrusted with hard calcareous sediments, algae, and shells."
            ],
            "damaged": [
                "🟠 A sardine can showing major deformations and a half-torn metal lid.",
                "🟠 This package shows pronounced red rust marks and very sharp edges.",
                "🟠 A metal can deformed by repeated impacts, containing a greasy black silt."
            ],
            "worn": [
                "🟡 A steel sardine can whole although dirty, showing superficial rust marks.",
                "🟡 This package has kept its characteristic rectangular shape despite some scratches.",
                "🟡 A used sardine can discarded recently, resting on a bed of sand at the bottom."
            ]
        },
        "fun_fact": "The sardine can is metal packaging made of tinplate or aluminum. If it still contains oil, its immersion causes the formation of a greasy film on the surface that blocks oxygen exchanges. In addition, its open and sharp metal structure constitutes a real physical danger for marine and freshwater animals, which can cut themselves or get trapped. It must be emptied of its residual oil and then thrown in the yellow recycling bin."
    },
    {
        "id": 180,
        "descriptions": {
            "badly damaged": [
                "🔴 An aluminum flask completely crushed and holed, its outer paint peeling off.",
                "🔴 This container shows a deformed metal neck and a broken plastic cap full of silt.",
                "🔴 Metal debris from a destroyed flask, half-buried under bottom gravel and silt."
            ],
            "damaged": [
                "🟠 An aluminum flask showing pronounced dents and deep abrasive scratches.",
                "🟠 This metal container shows a loss of outer color due to pebble friction.",
                "🟠 A sports flask deformed by violent impacts, containing fine silt and sand."
            ],
            "worn": [
                "🟡 A whole aluminum flask although dirty and showing superficial scratches.",
                "🟡 This metal container has kept its carrying strap despite signs of wear.",
                "🟡 A used sports flask discarded recently, resting on its side near the bank."
            ]
        },
        "fun_fact": "The dented aluminum flask is made of lightweight and durable aluminum, often lined inside with an epoxy resin varnish to prevent direct contact of the metal with drinks. Although aluminum is highly recyclable, its abandonment in nature is harmful. The metal oxidizes slowly, forming protective alumina, but the inner varnish can flake under the effect of physical erosion and release harmful organic compounds. It must be sorted in the yellow bin."
    },
    {
        "id": 181,
        "descriptions": {
            "badly damaged": [
                "🔴 A computer mouse whose plastic shell is broken into pieces and the optical sensor oxidized.",
                "🔴 This device shows printed circuits green from corrosion and buttons missing or blocked.",
                "🔴 Plastic and electronic component fragments from a broken mouse, encrusted with black mud."
            ],
            "damaged": [
                "🟠 A computer mouse showing major cracks on its plastic shell and scratches.",
                "🟠 This object shows a scroll wheel blocked by clay deposits and silt.",
                "🟠 A wireless mouse deformed by a violent mechanical impact, containing sand under its cover."
            ],
            "worn": [
                "🟡 A wireless computer mouse whole although dirty, resting near the bank.",
                "🟡 This device has kept its plastic shell intact despite minor signs of oxidation.",
                "🟡 A used computer mouse discarded recently, resting at the bottom of the clear water."
            ]
        },
        "fun_fact": "The wireless computer mouse is waste electrical and electronic equipment (WEEE). It contains a rigid ABS shell, an LED or laser optical sensor, micro-switches, a metal-rich printed circuit board, and a battery compartment. In the event of prolonged immersion, water infiltration causes complex oxidation reactions that release toxic substances into the aquatic environment (lead, cadmium from solder). It must join the WEEE recycling bin."
    },
    {
        "id": 182,
        "descriptions": {
            "badly damaged": [
                "🔴 An electrical charger casing whose plastic shell has opened, revealing the rusted transformer.",
                "🔴 This charger shows brass connection prongs completely bent and oxidized green.",
                "🔴 Remains of burnt plastic and electrical wires from a destroyed charger, buried in the silt."
            ],
            "damaged": [
                "🟠 A phone charger showing cracks on its hard plastic shell and bent prongs.",
                "🟠 This device shows green corrosion marks on its wet metal components.",
                "🟠 A charger deformed by mechanical crushing under pebbles, containing fine silt."
            ],
            "worn": [
                "🟡 A cell phone charger whole although dirty, with its plastic shell crack-free.",
                "🟡 This electrical adapter has kept its brass prongs usable despite scratches.",
                "🟡 A used charger discarded recently in the canal, resting at the bottom of the water."
            ]
        },
        "fun_fact": "The cell phone charger is an electronic AC adapter composed of a flame-retardant ABS plastic housing, a voltage transformer (copper winding and iron core), and semiconductor components mounted on a PCB. Falling to the bottom of a waterway exposes the aquatic environment to leaks of heavy metals and brominated flame retardant plastic additives, which are toxic and persistent. It must be deposited in the WEEE recycling bin."
    }
]

apply_updates(updates_fr, updates_en)
print("Batch 4 updated successfully!")
