# -*- coding: utf-8 -*-
from update_junk_util import apply_updates

updates_fr = [
    {
        "id": 183,
        "descriptions": {
            "badly damaged": [
                "🔴 Un lecteur MP3 dont l'écran en plastique est brisé et la batterie gonflée de liquide chimique.",
                "🔴 Cet appareil audio miniature montre des circuits électroniques rongés par l'oxydation de l'eau salée.",
                "🔴 Des débris de plastique et de boutons poussoirs d'un baladeur détruit, gisant au fond du canal."
            ],
            "damaged": [
                "🟠 Un lecteur MP3 présentant de profondes rayures sur sa coque métallique et des touches bloquées.",
                "🟠 Cet appareil montre des traces de corrosion verte sur ses prises de connexion et des fêlures.",
                "🟠 Un baladeur déformé par l'écrasement mécanique, contenant de la vase fine sous son écran."
            ],
            "worn": [
                "🟡 Un lecteur MP3 entier bien que très sale, avec sa prise d'écouteurs libre de rouille.",
                "🟡 Cet appareil a conservé ses boutons plastiques d'origine malgré des éraflures superficielles.",
                "🟡 Un lecteur MP3 usagé jeté récemment, reposant sur le sable près de la berge."
            ]
        },
        "fun_fact": "Le lecteur MP3 HS contient un boîtier en aluminium ou en plastique ABS, un écran à cristaux liquides (LCD), une carte électronique multicouche et une batterie rechargeable au lithium-ion. Son séjour prolongé dans l'eau présente un risque chimique très important. La batterie au lithium peut fuir, libérant des solvants organiques inflammables et des métaux toxiques comme le lithium, le cobalt et le manganèse. Ces métaux s'accumulent dans les tissus des organismes aquatiques, perturbant leur croissance. Il doit être trié dans la poubelle grise."
    },
    {
        "id": 184,
        "descriptions": {
            "badly damaged": [
                "🔴 Une calculatrice scientifique dont l'écran LCD est brisé en morceaux noirs et le clavier fondu.",
                "🔴 Cet appareil de calcul montre un circuit électronique vert rongé par l'eau et des touches manquantes.",
                "🔴 Des morceaux de plastique rigide et de membrane de touches de calculatrice, enfouis dans la boue."
            ],
            "damaged": [
                "🟠 Une calculatrice présentant des fissures importantes sur sa coque en plastique dur et des rayures.",
                "🟠 Cet appareil de calcul montre un petit panneau solaire brisé et des touches collées par du limon.",
                "🟠 Une calculatrice scientifique déformée par la pression de fond, contenant du sable fin."
            ],
            "worn": [
                "🟡 Une calculatrice scientifique entière bien que très sale, avec son couvercle de protection manquant.",
                "🟡 Cet outil de calcul a conservé ses touches en plastique lisibles malgré des éraflures.",
                "🟡 Une calculatrice usagée jetée récemment dans le canal, reposant au fond près d'une pierre."
            ]
        },
        "fun_fact": "La calculatrice scientifique est composée d'un boîtier externe en polystyrène ou en ABS, de touches souples en silicone, d'un écran à cristaux liquides et d'une carte électronique alimentée par une pile bouton (contenant souvent du mercure, du lithium ou de l'oxyde d'argent) ou par une petite cellule photovoltaïque. Si elle est abandonnée dans l'eau, la pile bouton finit par se corroder et libérer des métaux lourds hautement toxiques qui empoisonnent les poissons et la microfaune benthique. Elle doit être déposée dans la poubelle grise de tri."
    },
    {
        "id": 185,
        "descriptions": {
            "badly damaged": [
                "🔴 Une console de jeux portable dont l'écran est éclaté en morceaux coupants et la batterie gonflée.",
                "🔴 Cet appareil de jeu montre des boutons directionnels arrachés et un circuit électronique oxydé vert.",
                "🔴 Des débris de plastique rigide et de carte mère de console brisée, gisant dans le limon argileux."
            ],
            "damaged": [
                "🟠 Une console portable présentant des fissures majeures sur sa carrosserie plastique et des rayures.",
                "🟠 Cet appareil montre des ports de cartouche encrassés par la vase et une charnière brisée.",
                "🟠 Une console déformée par un choc mécanique violent sous l'eau, contenant du sable fin."
            ],
            "worn": [
                "🟡 Une console de jeux portable entière bien que sale, avec ses boutons de commande encore en place.",
                "🟡 Cet appareil a conservé son compartiment à piles fermé malgré des traces d'usure de surface.",
                "🟡 Une console de jeux usagée jetée récemment, reposant sur un banc de sable au fond."
            ]
        },
        "fun_fact": "La console de jeux portable cassée est un déchet technologique contenant du plastique ABS, un écran LCD rétroéclairé, des boutons en silicone conducteur, une carte électronique et une batterie lithium-polymère. L'infiltration d'eau douce ou salée oxyde les circuits et détruit la batterie, libérant du lithium et des métaux lourds (plomb, cuivre, étain). Ces polluants chimiques altèrent le système nerveux et la reproduction de la faune aquatique. Elle doit être ramenée dans un point de collecte ou jetée dans la poubelle grise."
    },
    {
        "id": 186,
        "descriptions": {
            "badly damaged": [
                "🔴 Un disque dur externe dont la coque métallique est percée par la rouille, le disque interne tordu.",
                "🔴 Cet équipement de stockage montre des ports de connexion USB complètement rongés de vert-de-gris.",
                "🔴 Des résidus d'acier, de plastique et de plateaux magnétiques brisés, enfouis sous le gravier de fond."
            ],
            "damaged": [
                "🟠 Un disque dur externe présentant des enfoncements majeurs et des traces de rouille rousse.",
                "🟠 Ce boîtier montre des rayures profondes et des infiltrations d'eau visibles sous le plastique protecteur.",
                "🟠 Un périphérique informatique déformé par l'écrasement, contenant de la vase grise et du limon."
            ],
            "worn": [
                "🟡 Un disque dur externe en aluminium entier bien que sale, avec sa coque sans fissures.",
                "🟡 Cet appareil a conservé sa prise de connexion métallique malgré des traces d'oxydation légères.",
                "🟡 Un disque dur usagé jeté récemment dans le canal, reposant au fond près de la berge."
            ]
        },
        "fun_fact": "Le disque dur externe rouillé est constitué d'un boîtier en aluminium ou en plastique, d'une carte de contrôle électronique et d'un bloc mécanique étanche abritant des plateaux magnétiques en aluminium ou en verre recouverts de cobalt et de terres rares (comme le néodyme des aimants de lecture). Sa dégradation libère des oxydes de fer et de cobalt qui polluent le sol sous-marin et affectent la faune benthique (mollusques, vers). Il doit être trié et valorisé dans le bac des DEEE de la poubelle grise."
    },
    {
        "id": 187,
        "descriptions": {
            "badly damaged": [
                "🔴 Un pèse-personne dont le plateau en verre trempé est brisé en milliers de morceaux coupants.",
                "🔴 Cet appareil de mesure montre des capteurs métalliques rongés par la rouille et des circuits oxydés.",
                "🔴 Des débris de verre poli et de plastique de pèse-personne brisé, mêlés à de la vase putride."
            ],
            "damaged": [
                "🟠 Un pèse-personne présentant des fêlures majeures sur sa structure en plastique et des rayures.",
                "🟠 Cet appareil électronique montre un écran digital brisé et des compartiments de piles oxydés.",
                "🟠 Un outil de mesure déformé par des chocs violents contre les rochers, contenant du sable fin."
            ],
            "worn": [
                "🟡 Un pèse-personne électronique entier bien que sale, avec son plateau en verre non fissuré.",
                "🟡 Cet appareil a conservé ses pieds en caoutchouc d'origine malgré des traces d'usure superficielles.",
                "🟡 Un pèse-personne usagé jeté récemment, reposant à plat sur le lit sablonneux de la rivière."
            ]
        },
        "fun_fact": "Le pèse-personne électronique associe un plateau en verre de sécurité trempé, une structure en plastique ABS, des capteurs de force métalliques (jauges de contrainte en acier et cuivre) et un écran LCD. Sa dégradation dans l'eau crée un double danger : la dispersion de débris de verre trempé tranchants et la libération de substances chimiques (cristaux liquides de l'écran, lithium ou métaux lourds des piles de type CR2032 souvent oubliées à l'intérieur). Il doit être recyclé dans le bac des DEEE de la poubelle grise."
    },
    {
        "id": 188,
        "descriptions": {
            "badly damaged": [
                "🔴 Un grille-pain complètement calciné et rouillé, dont la carrosserie en plastique a fondu en coulures noires.",
                "🔴 Cet appareil ménager brisé montre des fils de résistance en nickel-chrome coupés et verdis par l'eau.",
                "🔴 Une carcasse métallique de grille-pain remplie de vase noire charbonneuse et d'odeur de brûlé."
            ],
            "damaged": [
                "🟠 Un grille-pain présentant de larges taches de rouille brune sur ses flancs en acier poli.",
                "🟠 Cet appareil de cuisine montre un levier mécanique bloqué par la vase et des parois déformées.",
                "🟠 Un grille-pain déformé par l'écrasement mécanique sous l'eau, contenant du sable et du limon."
            ],
            "worn": [
                "🟡 Un grille-pain électrique entier bien que sale, avec sa carrosserie présentant quelques taches de rouille.",
                "🟡 Cet appareil a gardé son tiroir ramasse-miettes métallique malgré des éraflures superficielles.",
                "🟡 Un grille-pain usagé jeté récemment, dérivant près du fond avec des algues collées aux fentes."
            ]
        },
        "fun_fact": "Le grille-pain carbonisé est un petit appareil électroménager constitué d'une enveloppe extérieure en acier chromé ou en plastique ABS, d'un mécanisme de levier en acier et d'éléments chauffants constitués de fils en alliage nickel-chrome (nichrome) enroulés sur des plaques de mica. Dans l'eau, l'acier subit une corrosion rapide. Les métaux lourds constitutifs des résistances (chrome, nickel) s'oxydent lentement et se libèrent sous des formes solubles toxiques pour la faune. Il doit être jeté dans la poubelle grise."
    },
    {
        "id": 189,
        "descriptions": {
            "badly damaged": [
                "🔴 Une bouilloire dont la résistance chauffante est complètement recouverte de rouille et de calcaire épais.",
                "🔴 Cet appareil montre une coque en plastique fendue de haut en bas et un cordon électrique arraché.",
                "🔴 Une carcasse de bouilloire en acier déformée et encroûtée de limon noir vaseux et de coquillages."
            ],
            "damaged": [
                "🟠 Une bouilloire électrique présentant des rayures abrasives profondes et un couvercle cassé.",
                "🟠 Cet appareil ménager montre un socle électrique encrassé par des dépôts argileux et de la vase.",
                "🟠 Une bouilloire en plastique déformée par le ressac, contenant du sable grossier et des algues."
            ],
            "worn": [
                "🟡 Une bouilloire électrique en plastique blanc entière bien que sale, avec son filtre calcaire intact.",
                "🟡 Cet appareil a conservé sa poignée en plastique rigide malgré quelques éraflures de surface.",
                "🟡 Une bouilloire usagée jetée récemment dans le canal, reposant au fond près de la berge."
            ]
        },
        "fun_fact": "La bouilloire électrique réunit un corps en acier inoxydable ou en plastique PP, une résistance de chauffage blindée en cuivre nickelé ou en acier, et un interrupteur bimétallique. Sa dégradation dans le milieu aquatique est préjudiciable : le plastique PP résiste indéfiniment en libérant des additifs chimiques stabilisants, tandis que la résistance chauffante libère du nickel et du cuivre en s'oxydant, deux métaux hautement toxiques pour la flore et la faune des rivières. Elle doit être recyclée dans la poubelle grise."
    },
    {
        "id": 190,
        "descriptions": {
            "badly damaged": [
                "🔴 Un sèche-cheveux dont la turbine en plastique interne est brisée et le moteur électrique oxydé rouillé.",
                "🔴 Cet appareil montre une coque en plastique cassée en deux et des résistances verdis par le sel.",
                "🔴 Une carcasse de sèche-cheveux encroûtée d'algues filamenteuses et remplie de vase noire fétide."
            ],
            "damaged": [
                "🟠 Un sèche-cheveux présentant des fissures sur sa carrosserie plastique et une grille arrière encrassée.",
                "🟠 Cet appareil montre un interrupteur bloqué par des sédiments fins et un câble électrique coupé.",
                "🟠 Un sèche-cheveux déformé par l'écrasement mécanique sous l'eau, contenant du sable grossier."
            ],
            "worn": [
                "🟡 Un sèche-cheveux entier bien que sale, avec son câble électrique encore attaché.",
                "🟡 Cet appareil a conservé ses boutons en plastique d'origine malgré des éraflures superficielles.",
                "🟡 Un sèche-cheveux usagé jeté récemment, reposant sur le flanc au fond du ruisseau."
            ]
        },
        "fun_fact": "Le sèche-cheveux en panne est un appareil composé d'un boîtier en plastique technique (souvent du polycarbonate ou de l'ABS) résistant à la chaleur, d'un moteur électrique à courant continu, d'une turbine en plastique et d'une résistance chauffante en nichrome. Dans l'eau, le moteur et les bobinages de cuivre subissent une corrosion galvanique active qui détruit l'appareil et disperse des ions de cuivre et de nickel toxiques. Le plastique extérieur mettra plusieurs siècles à s'éroder. Il doit être trié dans la poubelle grise."
    },
    {
        "id": 191,
        "descriptions": {
            "badly damaged": [
                "🔴 Un tire-bouchon en fonte complètement rouillé, dont la mèche hélicoïdale est brisée et inutilisable.",
                "🔴 Cet outil s'effrite en plaques d'oxyde de fer épaisses sous l'action corrosive marine.",
                "🔴 Une carcasse de tire-bouchon encroûtée de sédiments marins calcaires durs et de coquillages."
            ],
            "damaged": [
                "🟠 Un tire-bouchon métallique présentant des déformations importantes sur ses bras articulés.",
                "🟠 Cet outil montre des plaques de rouille brune et des articulations bloquées par la corrosion.",
                "🟠 Un équipement en fonte déformé par des chocs répétés contre des galets, couvert de vase."
            ],
            "worn": [
                "🟡 Un tire-bouchon en fonte noire entier bien que sale, avec sa mèche encore droite.",
                "🟡 Cet outil a conservé sa forme caractéristique malgré des traces de rouille superficielles.",
                "🟡 Un tire-bouchon usagé jeté récemment dans le canal, reposant au fond près de la berge."
            ]
        },
        "fun_fact": "Le tire-bouchon en fonte est fabriqué à partir de fonte de fer, un alliage métallique riche en carbone très sensible à l'oxydation en milieu humide. Au contact de l'eau et de l'oxygène, la fonte subit une corrosion active qui la transforme en oxyde de fer (rouille). Bien que ce processus libère du fer dans l'eau (qui n'est pas toxique à ces doses et peut même servir de nutriment à certaines algues), la présence du déchet représente un danger de blessure physique pour les animaux marins. Il doit aller en décharge."
    },
    {
        "id": 192,
        "descriptions": {
            "badly damaged": [
                "🔴 Une poêle dont le revêtement antiadhésif s'écaille entièrement en lambeaux grisâtres et toxiques.",
                "🔴 Cette poêle montre une poignée en plastique bakélite brisée et un fond en aluminium tordu rouillé.",
                "🔴 Un disque métallique de poêle déformé et encroûté de vase noire et de sédiments durs."
            ],
            "damaged": [
                "🟠 Une poêle antiadhésive présentant des rayures abrasives profondes qui révèlent l'aluminium intérieur.",
                "🟠 Cet ustensile montre une poignée de fixation desserrée et des déformations sur sa structure.",
                "🟠 Une poêle de cuisine déformée par le courant, contenant du sable fin et des débris d'algues."
            ],
            "worn": [
                "🟡 Une poêle de cuisine entière bien que sale, avec sa poignée en bakélite encore solide.",
                "🟡 Cet ustensile a conservé son aspect rond malgré des éraflures superficielles sur son fond.",
                "🟡 Une poêle de cuisine usagée jetée récemment, reposant à plat au fond de l'eau claire."
            ]
        },
        "fun_fact": "La poêle antiadhésive rayée pose un problème écologique majeur en raison de son revêtement antiadhésif en polytétrafluoroéthylène (PTFE), plus connu sous le nom de Téflon. Le PTFE appartient à la famille des PFAS, surnommés 'polluants éternels' car ils ne se dégradent jamais dans la nature. Les rayures et l'écaillage de la poêle libèrent des microparticules de PTFE qui contaminent l'eau et s'accumulent dans les organismes des poissons, causant des troubles métaboliques. Elle doit être jetée en décharge."
    }
]

updates_en = [
    {
        "id": 183,
        "descriptions": {
            "badly damaged": [
                "🔴 An MP3 player whose plastic screen is broken and battery swollen with chemical liquid.",
                "🔴 This miniature audio device shows electronic circuits eaten away by saltwater oxidation.",
                "🔴 Plastic and push-button fragments from a destroyed walkman, lying at the bottom of the canal."
            ],
            "damaged": [
                "🟠 An MP3 player showing deep scratches on its metal shell and blocked keys.",
                "🟠 This device shows green corrosion marks on its connection jacks and cracks.",
                "🟠 A walkman deformed by mechanical crushing, containing fine silt under its screen."
            ],
            "worn": [
                "🟡 An MP3 player whole although very dirty, with its headphone jack free of rust.",
                "🟡 This device has kept its original plastic buttons despite superficial scratches.",
                "🟡 A used MP3 player discarded recently, resting on the sand near the bank."
            ]
        },
        "fun_fact": "The broken MP3 player contains an aluminum or ABS plastic housing, a liquid crystal display (LCD), a multi-layer electronic card, and a rechargeable lithium-ion battery. Its prolonged stay in water presents a very significant chemical risk. The lithium battery can leak, releasing flammable organic solvents and toxic metals such as lithium, cobalt, and manganese. These metals accumulate in the tissues of aquatic organisms, disrupting their growth. It must be sorted in the gray bin."
    },
    {
        "id": 184,
        "descriptions": {
            "badly damaged": [
                "🔴 A scientific calculator whose LCD screen is broken into black pieces and keyboard melted.",
                "🔴 This calculating device shows a green electronic circuit eaten by water and missing keys.",
                "🔴 Rigid plastic fragments and calculator key membrane pieces, buried in the mud."
            ],
            "damaged": [
                "🟠 A calculator showing major cracks on its hard plastic shell and scratches.",
                "🟠 This calculating device shows a broken small solar panel and keys stuck with silt.",
                "🟠 A scientific calculator deformed by bottom pressure, containing fine sand."
            ],
            "worn": [
                "🟡 A scientific calculator whole although very dirty, with its protective cover missing.",
                "🟡 This calculating tool has kept its plastic keys readable despite scratches.",
                "🟡 A used calculator discarded recently in the canal, resting at the bottom near a stone."
            ]
        },
        "fun_fact": "The scientific calculator is composed of a polystyrene or ABS outer casing, soft silicone keys, a liquid crystal display, and an electronic board powered by a button cell battery (often containing mercury, lithium, or silver oxide) or a small photovoltaic cell. If abandoned in water, the button cell eventually corrodes and releases highly toxic heavy metals that poison fish and benthic microfauna. It must be deposited in the gray sorting bin."
    },
    {
        "id": 185,
        "descriptions": {
            "badly damaged": [
                "🔴 A handheld game console whose screen is shattered into sharp pieces and battery swollen.",
                "🔴 This gaming device shows torn directional buttons and an electronic circuit oxidized green.",
                "🔴 Rigid plastic fragments and motherboard pieces from a broken console, lying in clayey silt."
            ],
            "damaged": [
                "🟠 A handheld console showing major cracks on its plastic body and scratches.",
                "🟠 This device shows cartridge ports clogged with silt and a broken hinge.",
                "🟠 A console deformed by a violent mechanical impact underwater, containing fine sand."
            ],
            "worn": [
                "🟡 A handheld game console whole although dirty, with its control buttons still in place.",
                "🟡 This device has kept its battery compartment closed despite surface wear marks.",
                "🟡 A used game console discarded recently, resting on a sandbank at the bottom."
            ]
        },
        "fun_fact": "The broken handheld game console is technological waste containing ABS plastic, a backlit LCD screen, conductive silicone buttons, an electronic board, and a lithium-polymer battery. Infiltration of fresh or salt water oxidizes the circuits and destroys the battery, releasing lithium and heavy metals (lead, copper, tin). These chemical pollutants alter the nervous system and reproduction of aquatic fauna. It must be returned to a collection point or thrown in the gray bin."
    },
    {
        "id": 186,
        "descriptions": {
            "badly damaged": [
                "🔴 An external hard drive whose metal shell is pierced by rust, the internal disk bent.",
                "🔴 This storage equipment shows USB connection ports completely eaten by verdigris.",
                "🔴 Steel, plastic, and broken magnetic platter residues, buried under bottom gravel."
            ],
            "damaged": [
                "🟠 An external hard drive showing major dents and red rust marks.",
                "🟠 This casing shows deep scratches and water infiltration visible under the protective plastic.",
                "🟠 A computer peripheral deformed by crushing, containing gray silt and mud."
            ],
            "worn": [
                "🟡 A whole aluminum external hard drive although dirty, with its casing crack-free.",
                "🟡 This device has kept its metal connection jack despite light signs of oxidation.",
                "🟡 A used hard drive discarded recently in the canal, resting at the bottom near the bank."
            ]
        },
        "fun_fact": "The rusted external hard drive consists of an aluminum or plastic case, an electronic control card, and a sealed mechanical block housing magnetic platters made of aluminum or glass coated with cobalt and rare earths (like the neodymium of the read magnets). Its degradation releases oxides of iron and cobalt that pollute the underwater soil and affect benthic fauna (mollusks, worms). It must be sorted and valued in the WEEE bin of the gray bin."
    },
    {
        "id": 187,
        "descriptions": {
            "badly damaged": [
                "🔴 A scale whose tempered glass platform is shattered into thousands of sharp pieces.",
                "🔴 This measuring device shows metal sensors eaten by rust and oxidized circuits.",
                "🔴 Tempered glass shards and plastic scale fragments mixed with putrid silt."
            ],
            "damaged": [
                "🟠 A scale showing major cracks on its plastic structure and scratches.",
                "🟠 This electronic scale shows a broken digital screen and oxidized battery compartments.",
                "🟠 A measuring tool deformed by violent impacts against rocks, containing fine sand."
            ],
            "worn": [
                "🟡 An electronic scale whole although dirty, with its glass platform uncracked.",
                "🟡 This device has kept its original rubber feet despite superficial signs of wear.",
                "🟡 A used scale discarded recently, resting flat on the sandy bed of the river."
            ]
        },
        "fun_fact": "The electronic scale combines a tempered safety glass platform, an ABS plastic structure, metal force sensors (steel and copper strain gauges), and an LCD screen. Its degradation in water creates a double danger: the dispersion of sharp tempered glass fragments and the release of chemical substances (liquid crystals from the screen, lithium or heavy metals from CR2032 batteries often forgotten inside). It must be recycled in the WEEE bin of the gray bin."
    },
    {
        "id": 188,
        "descriptions": {
            "badly damaged": [
                "🔴 A toaster completely charred and rusted, its plastic body melted into black drips.",
                "🔴 This broken kitchen appliance shows nickel-chromium heating elements cut and green from water.",
                "🔴 A metal toaster casing filled with charcoal-black silt and smelling of burning."
            ],
            "damaged": [
                "🟠 A toaster showing large brown rust spots on its polished steel sides.",
                "🟠 This kitchen appliance shows a mechanical lever blocked by silt and deformed walls.",
                "🟠 A toaster deformed by mechanical crushing under water, containing sand and silt."
            ],
            "worn": [
                "🟡 An electric toaster whole although dirty, its body showing some rust spots.",
                "🟡 This appliance has kept its metal crumb tray despite superficial scratches.",
                "🟡 A used toaster discarded recently, drifting near the bottom with algae stuck to the slots."
            ]
        },
        "fun_fact": "The charred toaster is a small household appliance consisting of a chrome-plated steel or ABS plastic outer shell, a steel lever mechanism, and heating elements made of nickel-chromium alloy (nichrome) wires wound on mica plates. In water, steel undergoes rapid corrosion. The heavy metals in the resistors (chromium, nickel) oxidize slowly and release toxic soluble forms into the fauna. It must be thrown in the gray bin."
    },
    {
        "id": 189,
        "descriptions": {
            "badly damaged": [
                "🔴 A kettle whose heating element is completely covered in rust and thick scale.",
                "🔴 This appliance shows a plastic shell split from top to bottom and a torn electrical cord.",
                "🔴 An steel kettle casing deformed and encrusted with muddy black silt and shells."
            ],
            "damaged": [
                "🟠 An electric kettle showing deep abrasive scratches and a broken lid.",
                "🟠 This household appliance shows an electrical base clogged with clay deposits and silt.",
                "🟠 A plastic kettle deformed by the backwash, containing coarse sand and algae."
            ],
            "worn": [
                "🟡 A white plastic electric kettle whole although dirty, with its scale filter intact.",
                "🟡 This appliance has kept its rigid plastic handle despite some surface scratches.",
                "🟡 A used kettle discarded recently in the canal, resting at the bottom near the bank."
            ]
        },
        "fun_fact": "The electric kettle combines a stainless steel or PP plastic body, a shielded copper-nickel or steel heating element, and a bimetallic switch. Its degradation in the aquatic environment is harmful: PP plastic resists indefinitely, releasing stabilizing chemical additives, while the heating resistor releases nickel and copper as it oxidizes, two metals highly toxic to river flora and fauna. It must be recycled in the gray bin."
    },
    {
        "id": 190,
        "descriptions": {
            "badly damaged": [
                "🔴 A hair dryer whose internal plastic fan is broken and electrical motor rusted.",
                "🔴 This device shows a plastic shell broken in two and heating resistors green from salt.",
                "🔴 A hair dryer casing encrusted with filamentous algae and filled with foul black silt."
            ],
            "damaged": [
                "🟠 A hair dryer showing cracks on its plastic body and a clogged rear grille.",
                "🟠 This appliance shows a switch blocked by fine sediments and a cut power cord.",
                "🟠 A hair dryer deformed by mechanical crushing under water, containing coarse sand."
            ],
            "worn": [
                "🟡 A hair dryer whole although dirty, with its power cord still attached.",
                "🟡 This appliance has kept its original plastic buttons despite superficial scratches.",
                "🟡 A used hair dryer discarded recently, resting on its side at the bottom of the stream."
            ]
        },
        "fun_fact": "The broken hair dryer is a device composed of a heat-resistant technical plastic housing (often polycarbonate or ABS), a DC electric motor, a plastic fan, and a nichrome heating resistor. In water, the motor and copper windings undergo active galvanic corrosion that destroys the device and disperses toxic copper and nickel ions. The outer plastic will take several centuries to erode. It must be sorted in the gray bin."
    },
    {
        "id": 191,
        "descriptions": {
            "badly damaged": [
                "🔴 A highly rusted cast iron corkscrew, its helical screw broken and unusable.",
                "🔴 This tool crumbles into thick iron oxide plates under corrosive marine action.",
                "🔴 A cast iron corkscrew casing encrusted with hard calcareous marine sediments and shells."
            ],
            "damaged": [
                "🟠 A metal corkscrew showing significant deformations on its articulated arms.",
                "🟠 This tool shows brown rust plates and joints blocked by corrosion.",
                "🟠 A cast iron gear deformed by repeated impacts against pebbles, covered in silt."
            ],
            "worn": [
                "🟡 A whole black cast iron corkscrew although dirty, with its worm screw still straight.",
                "🟡 This tool has kept its characteristic shape despite superficial rust marks.",
                "🟡 A used corkscrew discarded recently in the canal, resting at the bottom near the bank."
            ]
        },
        "fun_fact": "The cast iron corkscrew is made from cast iron, a carbon-rich metal alloy very sensitive to oxidation in a humid environment. In contact with water and oxygen, cast iron undergoes active corrosion that transforms it into iron oxide (rust). Although this process releases iron into the water (which is not toxic at these levels and can even serve as a nutrient for certain algae), the presence of the waste represents a danger of physical injury to marine animals. It must go to the landfill."
    },
    {
        "id": 192,
        "descriptions": {
            "badly damaged": [
                "🔴 A frying pan whose non-stick coating is peeling off in grayish, toxic shreds.",
                "🔴 This frying pan shows a broken bakelite plastic handle and a bent rusted aluminum bottom.",
                "🔴 A metal pan disc deformed and encrusted with black silt and hard sediments."
            ],
            "damaged": [
                "🟠 A non-stick pan showing deep abrasive scratches revealing the inner aluminum.",
                "🟠 This utensil shows a loose attachment handle and deformations on its structure.",
                "🟠 A kitchen frying pan deformed by the current, containing fine sand and algae debris."
            ],
            "worn": [
                "🟡 A kitchen frying pan whole although dirty, with its bakelite handle still strong.",
                "🟡 This utensil has kept its round appearance despite superficial scratches on its bottom.",
                "🟡 A kitchen frying pan discarded recently, resting flat at the bottom of the clear water."
            ]
        },
        "fun_fact": "The scratched non-stick pan poses a major ecological problem due to its non-stick coating made of polytetrafluoroethylene (PTFE), better known as Teflon. PTFE belongs to the PFAS family, nicknamed 'forever chemicals' because they never degrade in nature. Scratches and peeling of the pan release PTFE microparticles that contaminate the water and accumulate in the bodies of fish, causing metabolic disorders. It must be thrown in the landfill."
    }
]

apply_updates(updates_fr, updates_en)
print("Batch 5 updated successfully!")
