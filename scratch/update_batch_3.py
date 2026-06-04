# -*- coding: utf-8 -*-
from update_junk_util import apply_updates

updates_fr = [
    {
        "id": 163,
        "descriptions": {
            "badly damaged": [
                "🔴 Une monture en plastique noir complètement écrasée et brisée au niveau du pont, recouverte d'un limon fétide.",
                "🔴 Des branches de lunettes disjointes et tordues sous l'effet de courants violents, coincées dans des cailloux.",
                "🔴 Des éclats de résine plastique de lunettes cassées, jaunis par le soleil et encroûtés d'algues séchées."
            ],
            "damaged": [
                "🟠 Des lunettes de soleil en plastique montrant une charnière métallique rouillée et des branches asymétriques.",
                "🟠 Cette monture présente des rayures abrasives profondes sur toute sa surface plastique.",
                "🟠 Des lunettes sans verres déformées par le ressac, remplies de sédiments sableux fins."
            ],
            "worn": [
                "🟡 Une monture de lunettes de soleil entière bien que sale, reposant près de la rive.",
                "🟡 Ces lunettes en plastique noir ont conservé leurs deux branches mobiles malgré de légères éraflures.",
                "🟡 Des lunettes de soleil usagées jetées récemment, dont les verres ont été retirés ou perdus."
            ]
        },
        "fun_fact": "Les lunettes de soleil sans verres sont composées d'une monture généralement fabriquée en acétate de cellulose, en polycarbonate ou en nylon, assemblée avec de petites vis métalliques. En milieu aquatique, la monture en plastique résiste extrêmement bien à l'eau et mettra plusieurs siècles à se fragmenter. Sans ses verres protecteurs (qui peuvent être minéraux ou en plastique polycarbonate), elle devient un déchet plastique creux qui peut se coincer dans la gorge de grands poissons ou d'animaux aquatiques. Elle doit être triée en décharge."
    },
    {
        "id": 164,
        "descriptions": {
            "badly damaged": [
                "🔴 Une coque en silicone complètement déchirée en plusieurs lambeaux décolorés et gluants de vase noire.",
                "🔴 Cette protection de smartphone usée a perdu sa souplesse d'origine, devenue gluante et encroûtée de limon.",
                "🔴 Des morceaux de silicone rose fendillés et incrustés de micro-coquillages, enfouis dans la boue."
            ],
            "damaged": [
                "🟠 Une coque en silicone présentant des déchirures sur les coins et des décolorations grises.",
                "🟠 Cet étui souple montre des traces d'abrasion dues au sable et des parois intérieures encrassées.",
                "🟠 Une protection de téléphone déformée et étirée, contenant du limon et des dépôts organiques."
            ],
            "worn": [
                "🟡 Une coque de smartphone en silicone translucide entière bien que jaunie par la lumière solaire.",
                "🟡 Cet étui a conservé sa souplesse caractéristique malgré quelques rayures superficielles de surface.",
                "🟡 Une coque de protection usagée jetée récemment, flottant à la surface de l'eau claire."
            ]
        },
        "fun_fact": "La coque de smartphone en silicone est faite d'élastomère de silicone, un polymère synthétique réputé pour sa flexibilité et sa résistance thermique et chimique. Contrairement aux plastiques traditionnels, le silicone ne se décompose pas du tout dans l'eau et résiste très bien à la dégradation lumineuse. Il persiste ainsi indéfiniment dans les écosystèmes. S'il n'est pas mortel à court terme, il forme un déchet physique persistant qui modifie le substrat de fond s'il y est enfoui. Ce déchet doit être jeté en décharge."
    },
    {
        "id": 165,
        "descriptions": {
            "badly damaged": [
                "🔴 Une clé USB dont le connecteur métallique est complètement rouillé et tordu, la coque plastique fendue.",
                "🔴 Cet accessoire informatique montre un circuit intégré oxydé couvert de vert-de-gris et plein de vase.",
                "🔴 Des débris de plastique et de puces électroniques de clé USB brisée, collés au limon de fond."
            ],
            "damaged": [
                "🟠 Une clé USB présentant des fissures majeures sur sa coque en plastique et un connecteur encrassé.",
                "🟠 Ce périphérique de stockage montre des signes d'oxydation sur ses contacts dorés et des rayures.",
                "🟠 Une clé USB déformée par un choc mécanique violent, contenant du sable fin sous sa coque."
            ],
            "worn": [
                "🟡 Une clé USB entière bien que très sale, avec son capuchon de protection encore en place.",
                "🟡 Ce périphérique a conservé son connecteur métallique intact malgré des traces d'oxydation légères.",
                "🟡 Une clé USB usagée jetée récemment dans le canal, reposant au fond près de la berge."
            ]
        },
        "fun_fact": "La clé USB défectueuse est un déchet d'équipement électrique et électronique (DEEE). Elle contient une coque en plastique ou en métal, un connecteur en acier plaqué or ou nickel, et un circuit imprimé abritant une mémoire flash. Cette puce contient des métaux lourds et des métaux précieux (silicium, plomb, cuivre, or). Si elle est immergée, l'eau pénètre rapidement à l'intérieur et provoque une corrosion galvanique qui libère ces métaux lourds toxiques pour les poissons. Elle doit être triée dans le bac des DEEE gris."
    },
    {
        "id": 166,
        "descriptions": {
            "badly damaged": [
                "🔴 Une ampoule LED dont le globe en plastique est brisé en éclats pointus et le culot métallique oxydé.",
                "🔴 Cet élément d'éclairage montre des circuits électroniques internes rongés par le sel et couverts de boue.",
                "🔴 Une carcasse d'ampoule LED encroûtée de sédiments marins calcaires durs et de coquillages."
            ],
            "damaged": [
                "🟠 Une ampoule LED présentant de profondes fissures sur son diffuseur en plastique et un culot noirci.",
                "🟠 Cet objet montre des composants électroniques internes visibles à travers une fêlure de sa coque.",
                "🟠 Une ampoule déformée par des chocs répétés contre les rochers, contenant du sable fin."
            ],
            "worn": [
                "🟡 Une ampoule LED entière bien que sale, avec son culot en aluminium présentant des rayures de surface.",
                "🟡 Cet élément lumineux a conservé ses diodes intactes malgré des éraflures sur son diffuseur plastique.",
                "🟡 Une ampoule LED usagée jetée récemment, reposant sur un banc de limon près de la rive."
            ]
        },
        "fun_fact": "L'ampoule LED grillée comporte plusieurs parties distinctes : un globe en polycarbonate, un dissipateur thermique en aluminium ou en céramique, et un circuit électronique composé de diodes et d'un pilote électrique. Contrairement aux anciennes ampoules à incandescence, les LED contiennent des métaux semi-conducteurs (comme le gallium, l'arsenic ou l'indium) et des résidus de plomb dans les soudures. Sa dégradation dans l'eau contamine chimiquement le biotope. Elle doit être jetée dans le bac de tri des ampoules gris."
    },
    {
        "id": 167,
        "descriptions": {
            "badly damaged": [
                "🔴 Des écouteurs filaires coupés en plusieurs morceaux, les fils de cuivre dénudés verdis par l'oxydation.",
                "🔴 Cet accessoire audio montre des oreillettes en plastique écrasées et des membranes internes pourries.",
                "🔴 Des débris de câbles en plastique et de cuivre emmêlés dans des racines d'arbres sous l'eau."
            ],
            "damaged": [
                "🟠 Des écouteurs filaires présentant des pliures nettes et des gaines plastiques fendues sur le câble.",
                "🟠 Ce fil audio montre des écouteurs encrassés par de la vase grise et des membranes déformées.",
                "🟠 Des écouteurs emmêlés formant un nœud serré contenant des sédiments sableux grossiers."
            ],
            "worn": [
                "🟡 Des écouteurs filaires entiers mais très emmêlés, dérivant lentement juste sous la surface.",
                "🟡 Ce câble audio a conservé sa prise jack métallique intacte malgré des éraflures sur sa gaine.",
                "🟡 Des écouteurs usagés jetés récemment, flottant au gré du courant dans l'eau claire."
            ]
        },
        "fun_fact": "Les écouteurs filaires emmêlés sont composés de fils conducteurs en cuivre ultra-fins enveloppés d'une gaine isolante en élastomère ou en PVC souple, reliés à de petits haut-parleurs dotés d'aimants en néodyme. Dans l'eau, la gaine plastique se fendille sous l'effet de l'érosion physique. Le cuivre s'oxyde alors en libérant du carbonate de cuivre vert (vert-de-gris), une substance hautement toxique pour la faune et la flore aquatiques. Ce déchet d'équipements électriques doit être déposé dans la poubelle grise."
    },
    {
        "id": 168,
        "descriptions": {
            "badly damaged": [
                "🔴 Une rallonge électrique dont la gaine en plastique est complètement arrachée, révélant des fils dénudés rouillés.",
                "🔴 Ce câble de rallonge montre des prises de connexion en plastique écrasées et remplies de vase noire fétide.",
                "🔴 Des morceaux de conducteur en cuivre vert oxydés et emmêlés dans de la végétation aquatique en décomposition."
            ],
            "damaged": [
                "🟠 Une rallonge électrique présentant des fissures majeures sur sa gaine isolante en PVC blanc.",
                "🟠 Ce câble montre des traces d'usure par frottement sur des rochers et des broches de prise tordues.",
                "🟠 Un câble électrique déformé par des torsions répétées, contenant du limon fin dans ses prises."
            ],
            "worn": [
                "🟡 Une rallonge électrique entière bien que sale, enroulée lâchement et flottant près de la berge.",
                "🟡 Ce cordon a conservé ses prises en plastique souple intactes malgré quelques rayures superficielles.",
                "🟡 Une rallonge électrique usagée jetée récemment, reposant sur un lit de galets de fond."
            ]
        },
        "fun_fact": "La rallonge électrique dénudée contient une âme en cuivre multibrins isolée par du PVC (chlorure de polyvinyle) coloré et protégée par une gaine externe robuste en PVC souple. Le PVC contient souvent des plastifiants appelés phtalates pour le rendre flexible. Dans l'eau, ces additifs s'échappent lentement et contaminent l'écosystème. De plus, les fils de cuivre exposés s'oxydent rapidement, formant des sels de cuivre algicides toxiques pour les poissons. Elle doit être jetée dans la poubelle grise."
    },
    {
        "id": 169,
        "descriptions": {
            "badly damaged": [
                "🔴 Une disquette en plastique rigide complètement éclatée, dont le disque magnétique interne est déchiré et froissé.",
                "🔴 Ce support de stockage montre un volet métallique coulissant arraché et rouillé, plein de boue.",
                "🔴 Des débris de plastique noir et de film magnétique encroûtés d'algues et de sable argileux."
            ],
            "damaged": [
                "🟠 Une disquette présentant des fêlures majeures sur sa coque carrée et un ressort de volet cassé.",
                "🟠 Ce boîtier montre des décolorations dues au soleil et des traces d'humidité intérieures grises.",
                "🟠 Une disquette déformée par une forte pression hydrostatique, contenant du limon fin."
            ],
            "worn": [
                "🟡 Une disquette 3.5 pouces entière bien que sale, reposant à plat sur le sable de fond.",
                "🟡 Ce support de stockage a conservé son étiquette en papier blanc collée bien que partiellement délavée.",
                "🟡 Une disquette usagée jetée récemment, dérivant lentement entre deux eaux."
            ]
        },
        "fun_fact": "La disquette 3.5 pouces est un déchet historique constitué d'une coque en plastique rigide (ABS) protégeant un disque souple en polyester recouvert d'une couche d'oxyde métallique magnétique (comme l'oxyde de fer). La coque ABS mettra plus de 500 ans à se détruire physiquement dans l'eau. Le film magnétique, quant à lui, libère en s'érodant de fines particules d'oxydes métalliques et de liants polymères qui encrassent les sédiments de fond de rivière. Elle doit rejoindre la benne de tri de la décharge."
    },
    {
        "id": 170,
        "descriptions": {
            "badly damaged": [
                "🔴 Un boîtier en plastique noir complètement brisé en éclats coupants et jauni par la lumière solaire.",
                "🔴 Ce boîtier de DVD montre un film plastique transparent extérieur déchiré en lanières pleines de vase.",
                "🔴 Des débris de plastique rigide et de jaquette en papier bouillie, enfouis dans la boue de fond."
            ],
            "damaged": [
                "🟠 Un boîtier de DVD présentant des charnières brisées et des fêlures sur sa face arrière.",
                "🟠 Ce boîtier montre de larges rayures abrasives de sable et un système de fixation central cassé.",
                "🟠 Un contenant en plastique déformé par l'écrasement mécanique, rempli d'une vase argileuse fine."
            ],
            "worn": [
                "🟡 Un boîtier de DVD en plastique entier bien que sale, flottant à plat à la surface de l'eau.",
                "🟡 Ce boîtier a conservé sa jaquette intérieure en papier bien que mouillée et décolorée.",
                "🟡 Un boîtier de disque usagé jeté récemment, reposant près d'un banc de sable."
            ]
        },
        "fun_fact": "Le boîtier de DVD cassé est fait de polypropylène (PP), un polymère thermoplastique robuste et bon marché. Bien que ce plastique soit techniquement recyclable, les boîtiers de DVD jetés dans la nature échappent aux circuits de valorisation. Dans les rivières et les lacs, le plastique se dégrade sous l'action mécanique de l'eau et des UV en formant des débris de microplastiques de formes irrégulières. Ces débris accumulent des polluants organiques persistants (POP). Il doit être évacué en décharge."
    },
    {
        "id": 171,
        "descriptions": {
            "badly damaged": [
                "🔴 Un livre dont les pages en papier sont réduites à une pâte de cellulose grise collante et informe.",
                "🔴 Ce livre montre une couverture en carton totalement délaminée et couverte de moisissures fétides.",
                "🔴 Des lambeaux de papier d'imprimerie collés à des brindilles et encroûtés de sédiments sableux noirs."
            ],
            "damaged": [
                "🟠 Un livre de poche présentant des pages gonflées d'eau et une reliure en colle synthétique brisée.",
                "🟠 Ce volume imprimé montre des textes effacés par la dissolution des encres d'imprimerie.",
                "🟠 Un livre déformé par l'humidité prolongée, contenant du sable fin entre ses pages jaunies."
            ],
            "worn": [
                "🟡 Un livre de poche entier bien que très mouillé, flottant lâchement à la surface de l'eau.",
                "🟡 Cet ouvrage a conservé sa couverture en papier souple identifiable malgré des pliures de surface.",
                "🟡 Un livre usagé perdu récemment, reposant sur le flanc au bord de l'eau douce."
            ]
        },
        "fun_fact": "Le livre de poche délavé est un déchet organique composé de papier (fibres de cellulose issues du bois), de colles de reliure en polyuréthane ou en EVA, et d'encres d'imprimerie chimiques. Bien que la cellulose se biodégrade naturellement en quelques semaines dans l'eau, les encres synthétiques utilisées pour l'impression contiennent souvent des pigments métalliques lourds et des solvants qui polluent localement le cours d'eau. La colle de reliure synthétique ne se dégrade pas et persiste. Il doit aller en décharge."
    },
    {
        "id": 172,
        "descriptions": {
            "badly damaged": [
                "🔴 Un cahier dont les feuilles de papier ont pourri en une pâte grise, la spirale métallique rouillée rompue.",
                "🔴 Ce cahier montre une couverture plastique déchirée et encroûtée de vase noire vaseuse.",
                "🔴 Des résidus de carton bouilli collés à des spires en acier oxydées, enfouis dans les sédiments de fond."
            ],
            "damaged": [
                "🟠 Un cahier à spirale présentant des pages collées entre elles par la vase et des fêlures sur sa couverture.",
                "🟠 Ce bloc de papier montre des traces de rouille rousse provenant de sa reliure métallique humide.",
                "🟠 Un cahier déformé par le courant d'eau, contenant des dépôts de sédiments sableux lourds."
            ],
            "worn": [
                "🟡 Un cahier à spirale entier bien que mouillé, avec ses spires métalliques encore régulières.",
                "🟡 Ce cahier a conservé sa couverture plastique colorée malgré quelques rayures superficielles.",
                "🟡 Un cahier de notes usagé jeté récemment dans le canal, reposant près de la berge de sable."
            ]
        },
        "fun_fact": "Le cahier à spirales rouillées est un déchet composite. Il réunit des feuilles de papier de cellulose, une spirale en acier plastifié ou en plastique rigide, et une couverture en plastique PP. La partie papier se dégrade rapidement dans l'eau douce. En revanche, le revêtement plastique de la spirale et la couverture persistent des centaines d'années, se fragmentant en microplastiques toxiques. La partie métallique s'oxyde en libérant de l'oxyde de fer. Il doit être déposé aux encombrants en décharge."
    }
]

updates_en = [
    {
        "id": 163,
        "descriptions": {
            "badly damaged": [
                "🔴 A black plastic frame completely crushed and broken at the bridge, covered with foul silt.",
                "🔴 Spectacle temples disjointed and twisted under the action of violent currents, wedged in stones.",
                "🔴 Shards of plastic resin from broken sunglasses, yellowed by the sun and encrusted with dried algae."
            ],
            "damaged": [
                "🟠 Plastic sunglasses showing a rusted metal hinge and asymmetrical temples.",
                "🟠 This frame shows deep abrasive scratches over its entire plastic surface.",
                "🟠 Sunglasses without lenses deformed by the surf, filled with fine sandy sediments."
            ],
            "worn": [
                "🟡 A whole sunglasses frame although dirty, resting near the riverbank.",
                "🟡 These black plastic sunglasses have kept their two movable temples despite minor scratches.",
                "🟡 Used sunglasses discarded recently, whose lenses have been removed or lost."
            ]
        },
        "fun_fact": "The sunglasses without lenses are made of a frame typically manufactured from cellulose acetate, polycarbonate, or nylon, assembled with tiny metal screws. In the aquatic environment, the plastic frame resists water extremely well and will take several centuries to fragment. Without its protective lenses (which can be mineral glass or polycarbonate plastic), it becomes a hollow plastic waste item that can get stuck in the throat of large fish or aquatic animals. It must be sorted in the landfill."
    },
    {
        "id": 164,
        "descriptions": {
            "badly damaged": [
                "🔴 A silicone case completely torn into several discolored shreds, slimy with black silt.",
                "🔴 This worn smartphone protection has lost its original flexibility, becoming sticky and encrusted with silt.",
                "🔴 Pink silicone pieces cracked and embedded with micro-shells, buried in the mud."
            ],
            "damaged": [
                "🟠 A silicone case showing tears on the corners and gray discoloration.",
                "🟠 This flexible cover shows signs of sand abrasion and dirty inner walls.",
                "🟠 A phone case deformed and stretched, containing silt and organic deposits."
            ],
            "worn": [
                "🟡 A translucent silicone smartphone case whole although yellowed by sunlight.",
                "🟡 This cover has kept its characteristic flexibility despite some superficial surface scratches.",
                "🟡 A used protective case discarded recently, floating on the surface of clear water."
            ]
        },
        "fun_fact": "The silicone smartphone case is made of silicone elastomer, a synthetic polymer renowned for its flexibility and thermal/chemical resistance. Unlike traditional plastics, silicone does not break down at all in water and resists light degradation very well. It thus persists indefinitely in ecosystems. While not immediately fatal, it forms a persistent physical waste that alters the bottom substrate if buried. This waste must be thrown in the landfill."
    },
    {
        "id": 165,
        "descriptions": {
            "badly damaged": [
                "🔴 A USB flash drive whose metal connector is completely rusted and bent, the plastic casing split.",
                "🔴 This IT accessory shows an oxidized integrated circuit covered with verdigris and filled with silt.",
                "🔴 Plastic and electronic chip fragments from a broken USB key, stuck to the bottom silt."
            ],
            "damaged": [
                "🟠 A USB flash drive showing major cracks on its plastic casing and a dirty connector.",
                "🟠 This storage peripheral shows signs of oxidation on its golden contacts and scratches.",
                "🟠 A USB flash drive deformed by a violent mechanical impact, containing fine sand under its casing."
            ],
            "worn": [
                "🟡 A USB flash drive whole although very dirty, with its protective cap still in place.",
                "🟡 This device has kept its metal connector intact despite light signs of oxidation.",
                "🟡 A used USB flash drive discarded recently in the canal, resting at the bottom near the bank."
            ]
        },
        "fun_fact": "The defective USB flash drive is waste electrical and electronic equipment (WEEE). It contains a plastic or metal shell, a gold or nickel-plated steel connector, and a printed circuit board housing a flash memory. This chip contains heavy metals and precious metals (silicon, lead, copper, gold). If submerged, water quickly penetrates inside and causes galvanic corrosion that releases these toxic heavy metals for fish. It must be sorted in the gray WEEE bin."
    },
    {
        "id": 166,
        "descriptions": {
            "badly damaged": [
                "🔴 A LED bulb whose plastic globe is broken into sharp shards and its metal base oxidized.",
                "🔴 This lighting element shows internal electronic circuits eroded by salt and covered with mud.",
                "🔴 A LED bulb casing encrusted with hard calcareous marine sediments and shells."
            ],
            "damaged": [
                "🟠 A LED bulb showing deep cracks on its plastic diffuser and a blackened base.",
                "🟠 This object shows internal electronic components visible through a crack in its casing.",
                "🟠 A bulb deformed by repeated impacts against rocks, containing fine sand."
            ],
            "worn": [
                "🟡 A LED bulb whole although dirty, its aluminum base showing surface scratches.",
                "🟡 This light element has kept its diodes intact despite scratches on its plastic diffuser.",
                "🟡 A used LED bulb discarded recently, resting on a silt bank near the riverbank."
            ]
        },
        "fun_fact": "The burnt-out LED bulb has several distinct parts: a polycarbonate globe, an aluminum or ceramic heat sink, and an electronic circuit composed of diodes and an electrical driver. Unlike old incandescent bulbs, LEDs contain semiconductor metals (such as gallium, arsenic, or indium) and lead residues in the solder. Its degradation in water chemically contaminates the biotope. It must be thrown in the gray bulb recycling bin."
    },
    {
        "id": 167,
        "descriptions": {
            "badly damaged": [
                "🔴 Wired earphones cut into several pieces, the bare copper wires green with oxidation.",
                "🔴 This audio accessory shows crushed plastic earbuds and rotted internal membranes.",
                "🔴 Cable plastic and copper fragments tangled in tree roots underwater."
            ],
            "damaged": [
                "🟠 Wired earphones showing sharp folds and split plastic sheaths on the cable.",
                "🟠 This audio cord shows earbuds clogged with gray silt and deformed membranes.",
                "🟠 Tangled earphones forming a tight knot containing coarse sandy sediments."
            ],
            "worn": [
                "🟡 Wired earphones whole but very tangled, drifting slowly just below the surface.",
                "🟡 This audio cable has kept its metal jack plug intact despite scratches on its sheath.",
                "🟡 Used earphones discarded recently, floating in the clear water current."
            ]
        },
        "fun_fact": "The tangled wired earphones are composed of ultra-fine copper conducting wires wrapped in an insulating sheath of elastomer or flexible PVC, connected to small speakers with neodymium magnets. In water, the plastic sheath cracks under mechanical erosion. The copper then oxidizes, releasing green copper carbonate (verdigris), a substance highly toxic to aquatic fauna and flora. This electrical waste must be deposited in the gray bin."
    },
    {
        "id": 168,
        "descriptions": {
            "badly damaged": [
                "🔴 An extension cord whose plastic sheath is completely torn off, revealing rusted bare wires.",
                "🔴 This extension cable shows crushed connection plugs filled with foul black silt.",
                "🔴 Green copper conductor pieces oxidized and tangled in decaying aquatic vegetation."
            ],
            "damaged": [
                "🟠 An extension cord showing major cracks on its white PVC insulating sheath.",
                "🟠 This cable shows wear marks from rubbing on rocks and bent plug prongs.",
                "🟠 An electrical cable deformed by repeated twisting, containing fine silt in its plugs."
            ],
            "worn": [
                "🟡 An extension cord whole although dirty, loosely wound and floating near the bank.",
                "🟡 This cord has kept its flexible plastic plugs intact despite some superficial scratches.",
                "🟡 A used extension cord discarded recently, resting on a bed of bottom pebbles."
            ]
        },
        "fun_fact": "The stripped extension cord contains a multi-strand copper core insulated by colored PVC (polyvinyl chloride) and protected by a robust outer sheath of flexible PVC. PVC often contains plasticizers called phthalates to make it flexible. In water, these additives escape slowly and contaminate the ecosystem. Furthermore, the exposed copper wires oxidize quickly, forming algicidal copper salts toxic to fish. It must be thrown in the gray bin."
    },
    {
        "id": 169,
        "descriptions": {
            "badly damaged": [
                "🔴 A rigid plastic floppy disk completely shattered, its internal magnetic disk torn and creased.",
                "🔴 This storage medium shows a sliding metal shutter torn off and rusted, full of mud.",
                "🔴 Black plastic and magnetic film debris encrusted with algae and clayey sand."
            ],
            "damaged": [
                "🟠 A floppy disk showing major cracks on its square casing and a broken shutter spring.",
                "🟠 This casing shows sun discoloration and gray internal moisture traces.",
                "🟠 A floppy disk deformed by high hydrostatic pressure, containing fine silt."
            ],
            "worn": [
                "🟡 A 3.5-inch floppy disk whole although dirty, resting flat on the bottom sand.",
                "🟡 This storage medium has kept its white paper label attached, although partially washed out.",
                "🟡 A used floppy disk discarded recently, drifting slowly between two water layers."
            ]
        },
        "fun_fact": "The 3.5-inch floppy disk is historical waste consisting of a rigid plastic shell (ABS) protecting a flexible disk made of polyester coated with a magnetic metal oxide layer (like iron oxide). The ABS shell will take more than 500 years to physically destroy itself in water. The magnetic film, as it erodes, releases fine particles of metal oxides and polymer binders that clog river bottom sediments. It must join the sorting bin at the landfill."
    },
    {
        "id": 170,
        "descriptions": {
            "badly damaged": [
                "🔴 A black plastic case completely broken into sharp shards and yellowed by sunlight.",
                "🔴 This DVD case shows an outer clear plastic film torn into strips full of silt.",
                "🔴 Rigid plastic fragments and boiled paper cover, buried in the bottom mud."
            ],
            "damaged": [
                "🟠 A DVD case showing broken hinges and cracks on its back cover.",
                "🟠 This case shows large sand abrasion scratches and a broken central hub system.",
                "🟠 A plastic container deformed by mechanical crushing, filled with fine clayey silt."
            ],
            "worn": [
                "🟡 A plastic DVD case whole although dirty, floating flat on the water surface.",
                "🟡 This case has kept its paper cover inside, although wet and discolored.",
                "🟡 A used disc case discarded recently, resting near a sandbank."
            ]
        },
        "fun_fact": "The broken DVD case is made of polypropylene (PP), a robust and cheap thermoplastic polymer. Although this plastic is technically recyclable, DVD cases thrown into nature escape recycling channels. In rivers and lakes, the plastic degrades under the mechanical action of water and UV, forming irregular microplastic debris. These debris accumulate persistent organic pollutants (POPs) from the water. It must be disposed of in the landfill."
    },
    {
        "id": 171,
        "descriptions": {
            "badly damaged": [
                "🔴 A book whose paper pages are reduced to a sticky, shapeless gray cellulose pulp.",
                "🔴 This book shows a cardboard cover completely delaminated and covered with foul mold.",
                "🔴 Print paper shreds stuck to twigs and encrusted with black sandy sediments."
            ],
            "damaged": [
                "🟠 A paperback book showing pages swollen with water and a broken synthetic glue binding.",
                "🟠 This printed volume shows texts erased by the dissolution of chemical printing inks.",
                "🟠 A book deformed by prolonged moisture, containing fine sand between its yellowed pages."
            ],
            "worn": [
                "🟡 A paperback book whole although very wet, floating loosely on the water surface.",
                "🟡 This work has kept its flexible paper cover identifiable despite surface folds.",
                "🟡 A used book lost recently, resting on its side at the edge of fresh water."
            ]
        },
        "fun_fact": "The faded paperback book is organic waste composed of paper (cellulose fibers from wood), synthetic binding glues made of polyurethane or EVA, and chemical printing inks. Although cellulose biodegrades naturally in a few weeks in water, the synthetic inks used for printing often contain heavy metal pigments and solvents that locally pollute the waterway. The synthetic binding glue does not degrade and persists. It must go to the landfill."
    },
    {
        "id": 172,
        "descriptions": {
            "badly damaged": [
                "🔴 A notebook whose paper sheets have rotted into a gray paste, the rusted metal coil broken.",
                "🔴 This notebook shows a torn plastic cover encrusted with slimy black silt.",
                "🔴 Boiled cardboard residues stuck to oxidized steel coils, buried in the bottom sediments."
            ],
            "damaged": [
                "🟠 A spiral notebook showing pages glued together by silt and cracks on its cover.",
                "🟠 This paper pad shows red rust marks from its wet metal binding coil.",
                "🟠 A notebook deformed by the water current, containing heavy sandy sediment deposits."
            ],
            "worn": [
                "🟡 A spiral notebook whole although wet, with its metal binding coils still regular.",
                "🟡 This notebook has kept its colored plastic cover despite some superficial scratches.",
                "🟡 A used notebook discarded recently in the canal, resting near the sandy bank."
            ]
        },
        "fun_fact": "The spiral notebook is a composite waste item. It combines cellulose paper sheets, a plastic-coated steel or rigid plastic spiral, and a PP plastic cover. The paper part decomposes quickly in fresh water. In contrast, the plastic coating of the spiral and the cover persist for hundreds of years, fragmenting into toxic microplastics. The metallic part oxidizes, releasing iron oxide. It must be deposited in the non-recyclable bin at the landfill."
    }
]

apply_updates(updates_fr, updates_en)
print("Batch 3 updated successfully!")
