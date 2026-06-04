# -*- coding: utf-8 -*-
from update_junk_util import apply_updates

updates_fr = [
    {
        "id": 203,
        "descriptions": {
            "badly damaged": [
                "🔴 Un sèche-linge complètement rouillé et écrasé, dont la carrosserie en tôle s'effrite en plaques d'oxyde de fer.",
                "🔴 Cet appareil montre un tambour en acier inoxydable bloqué et déformé, rempli de limon noir et d'algues.",
                "🔴 Une carcasse de sèche-linge encroûtée de sédiments marins calcaires durs et de coquillages."
            ],
            "damaged": [
                "🟠 Un sèche-linge présentant des déformations importantes sur ses flancs et une porte vitrée brisée.",
                "🟠 Cet appareil ménager montre des conduits internes en plastique fêlés et des circuits oxydés.",
                "🟠 Un sèche-linge déformé par l'écrasement mécanique sous l'eau, contenant du sable et de la vase."
            ],
            "worn": [
                "🟡 Un sèche-linge entier bien que sale, avec sa carrosserie en tôle d'acier ne présentant pas de fissure.",
                "🟡 Cet appareil a conservé son cordon d'alimentation au dos malgré des traces d'usure de surface.",
                "🟡 Un sèche-linge usagé jeté récemment, reposant sur le flanc au fond du canal."
            ]
        },
        "fun_fact": "Le sèche-linge tambour bloqué fait partie des gros équipements électroménagers (GEM) rejetés dans l'environnement. Il contient une quantité importante d'acier (carrosserie et tambour), un moteur de rotation lourd en cuivre et acier, des composants électroniques de commande, ainsi que des conduites et des bacs en plastique PP ou ABS. Dans les cours d'eau, sa présence perturbe la sédimentation et détruit le biotope local en écrasant la flore benthiques. La corrosion lente des métaux libère des métaux lourds nocifs. Il doit être déposé en décharge."
    },
    {
        "id": 204,
        "descriptions": {
            "badly damaged": [
                "🔴 Un chauffe-eau électrique complètement éventré et rouillé, dont l'isolant intérieur se détache.",
                "🔴 Cet énorme réservoir montre des parois métalliques percées par la corrosion et une cuve pleine de boue noire.",
                "🔴 Une carcasse de chauffe-eau encroûtée de sédiments durs et d'algues filamenteuses en décomposition."
            ],
            "damaged": [
                "🟠 Un chauffe-eau présentant des enfoncements majeurs sur sa coque en tôle et des tuyaux de cuivre tordus.",
                "🟠 Ce ballon électrique montre des traces de rouille rousse prononcées et un groupe de sécurité cassé.",
                "🟠 Un réservoir d'eau déformé par des courants de fond violents, contenant du sable et du limon fin."
            ],
            "worn": [
                "🟡 Un chauffe-eau électrique cylindrique complet bien que sale, présentant des éraflures superficielles.",
                "🟡 Cet appareil a conservé ses raccords filetés d'entrée et de sortie malgré des traces de rouille légères.",
                "🟡 Un chauffe-eau usagé jeté récemment, reposant à plat près d'un banc de galets."
            ]
        },
        "fun_fact": "Le chauffe-eau électrique (ou cumulus) est constitué d'une enveloppe externe en tôle d'acier peinte, d'une couche épaisse d'isolant en mousse de polyuréthane, d'une cuve interne en acier émaillé, d'une anode en magnésium ou titane, et d'une résistance blindée. La mousse de polyuréthane se dégrade dans l'eau en libérant des composés organiques volatils toxiques et des microplastiques persistants. La corrosion de la résistance libère du cuivre et du nickel, toxiques pour la vie aquatique. Il doit être recyclé via le circuit de la décharge."
    },
    {
        "id": 205,
        "descriptions": {
            "badly damaged": [
                "🔴 Un téléviseur dont l'écran plat est complètement brisé en éclats de verre fins et coupants.",
                "🔴 Cet appareil montre un châssis en plastique écrasé et des cartes électroniques rongées verdis.",
                "🔴 Des débris de plastique et de verre de téléviseur brisé, gisant dans le limon argileux noir."
            ],
            "damaged": [
                "🟠 Un téléviseur écran plat présentant des fissures majeures sur son boîtier en plastique dur et des rayures.",
                "🟠 Cet appareil électronique montre une dalle brisée et des traces d'humidité intérieures grises.",
                "🟠 Un téléviseur déformé par des chocs violents contre des rochers, contenant du sable fin."
            ],
            "worn": [
                "🟡 Un téléviseur écran plat entier bien que sale, reposant à plat au fond de l'eau.",
                "🟡 Cet appareil a conservé sa télécommande d'origine coincée dans son pied malgré des traces d'usure.",
                "🟡 Un téléviseur usagé jeté récemment, reposant près d'une rive sablonneuse."
            ]
        },
        "fun_fact": "Le téléviseur écran plat brisé est un déchet d'équipements électriques et électroniques (DEEE) particulièrement complexe. Il comporte une dalle LCD contenant des cristaux liquides (composés organiques fluorés nocifs) ou des diodes OLED, un rétroéclairage à LED, des diffuseurs optiques en PMMA, une carte électronique contenant du plomb, du cadmium et de l'arsenic, et un boîtier en plastique ignifugé. L'immersion de ces composants contamine l'eau et empoisonne la faune benthique de façon durable. Il doit être recyclé en décharge."
    },
    {
        "id": 206,
        "descriptions": {
            "badly damaged": [
                "🔴 Une unité centrale dont la carrosserie métallique est complètement rouillée et écrasée.",
                "🔴 Cet ordinateur montre des cartes mères internes et des disques durs rongés par l'oxydation de l'eau.",
                "🔴 Des débris de plastique, d'acier et de composants de circuit intégré de PC, encroûtés de boue noire."
            ],
            "damaged": [
                "🟠 Une unité centrale d'ordinateur présentant des charnières et des panneaux métalliques tordus.",
                "🟠 Ce boîtier montre de profondes rayures abrasives de sable et un lecteur de disque brisé.",
                "🟠 Un boîtier de PC déformé par l'écrasement mécanique, rempli d'une vase argileuse fine."
            ],
            "worn": [
                "🟡 Une unité centrale d'ordinateur entière bien que sale, avec ses ports USB externes encore réguliers.",
                "🟡 Ce boîtier a conservé ses ventilateurs intérieurs malgré des traces d'usure superficielles.",
                "🟡 Un ordinateur usagé jeté récemment dans le canal, reposant au fond près de la berge."
            ]
        },
        "fun_fact": "L'unité centrale d'ordinateur est un déchet électronique (DEEE) lourd qui renferme une grande variété de matériaux : acier galvanisé (châssis), plastique ABS, cartes imprimées en résine époxy et fibre de verre chargées de métaux lourds (plomb des soudures, cadmium, chrome) et de métaux précieux (or, argent, cuivre). Dans l'eau, les métaux s'oxydent et se dissolvent lentement sous forme d'ions métalliques hautement toxiques qui s'accumulent dans les sédiments de rivière. Il doit être trié en décharge."
    },
    {
        "id": 207,
        "descriptions": {
            "badly damaged": [
                "🔴 Une imprimante jet d'encre complètement écrasée et brisée, les cartouches d'encre éventrées.",
                "🔴 Cet appareil montre des circuits imprimés verdis par l'oxydation et des rouleaux d'alimentation cassés.",
                "🔴 Des débris de plastique rigide et de cartouches vides de jet d'encre, gisant dans le limon argileux."
            ],
            "damaged": [
                "🟠 Une imprimante présentant des fissures majeures sur sa carrosserie plastique et des charnières brisées.",
                "🟠 Cet appareil de bureau montre un scanner en verre fissuré et des coulures d'encre noire visibles.",
                "🟠 Une imprimante déformée par la pression de fond, contenant du sable fin et des algues."
            ],
            "worn": [
                "🟡 Une imprimante jet d'encre entière bien que sale, avec sa vitre de scanner non fissurée.",
                "🟡 Cet appareil de bureau a conservé ses boutons en plastique d'origine malgré des éraflures.",
                "🟡 Une imprimante usagée jetée récemment dans le canal, reposant près d'une pierre."
            ]
        },
        "fun_fact": "L'imprimante jet d'encre contient des plastiques techniques (ABS, polystyrène), des moteurs électriques, des axes en acier, des cartes électroniques et des cartouches d'encre liquide. L'encre jet d'encre contient des colorants organiques ou des pigments métalliques, des solvants (glycols) et des agents conservateurs toxiques. Si l'imprimante est immergée, ces encres se dissolvent instantanément, créant une nappe toxique qui empoisonne les organismes filtreurs et les poissons locaux. Elle doit rejoindre la décharge."
    },
    {
        "id": 208,
        "descriptions": {
            "badly damaged": [
                "🔴 Un four à pizza de jardin en briques complètement écroulé en un tas de gravats et de mortier.",
                "🔴 Ce four montre une sole en pierre réfractaire brisée et une porte en acier complètement rouillée.",
                "🔴 Des débris de briques réfractaires cuites et de suie noire charbonneuse, enfouis dans la boue."
            ],
            "damaged": [
                "🟠 Un four à pizza présentant des fissures importantes sur son dôme en briques et du mortier effrité.",
                "🟠 Cet équipement montre des traces prononcées de suie sur ses parois et une structure déformée.",
                "🟠 Un four de jardin déformé par l'écrasement mécanique sous l'eau, contenant du sable et du limon."
            ],
            "worn": [
                "🟡 Un four à pizza de jardin en briques entier bien que sale, avec son dôme encore stable.",
                "🟡 Cet équipement a conservé sa porte en acier malgré des traces de rouille superficielles.",
                "🟡 Un four de jardin usagé jeté récemment, reposant au fond près d'une berge du canal."
            ]
        },
        "fun_fact": "Le four à pizza de jardin est composé de briques d'argile cuite réfractaire, de mortier de ciment ou de chaux, de sable de quartz, de laine de roche (pour l'isolation) et de structures métalliques en acier ou fonte. Dans l'eau, les matériaux minéraux (briques, mortier) sont relativement inertes et ne polluent pas chimiquement, mais leur écroulement détruit physiquement le biotope de fond en ensevelissant la faune benthique sous un tas de gravats lourds. Il doit être trié dans la benne tout-venant en décharge."
    },
    {
        "id": 209,
        "descriptions": {
            "badly damaged": [
                "🔴 Une carcasse de tondeuse complètement rouillée et tordue, dont le moteur thermique est Eventré.",
                "🔴 Cet appareil de jardinage montre des roues en plastique cassées et un carter métallique percé.",
                "🔴 Une carcasse de tondeuse encroûtée de vase noire, de calcaire et de coquilles maritimes."
            ],
            "damaged": [
                "🟠 Une carcasse de tondeuse à gazon présentant des déformations sur son carter en acier rouillé.",
                "🟠 Cet appareil montre une lame métallique tordue par des chocs violents et un guidon cassé.",
                "🟠 Une tondeuse déformée par l'écrasement de fond, contenant de la vase fine et du sable."
            ],
            "worn": [
                "🟡 Une carcasse de tondeuse à gazon entière bien que sale, avec ses roues encore mobiles.",
                "🟡 Cet appareil a conservé son réservoir d'essence en plastique malgré des éraflures superficielles.",
                "🟡 Une tondeuse usagée jetée récemment dans le canal, reposant au fond de l'eau claire."
            ]
        },
        "fun_fact": "La carcasse de tondeuse à gazon comporte un carter en acier ou en aluminium, des roues en plastique et un moteur à essence à quatre temps. Si elle contient encore de l'essence ou de l'huile moteur, son immersion est catastrophique pour l'écosystème aquatique : les hydrocarbures forment un film imperméable en surface bloquant l'oxygène atmosphérique et étouffant les poissons. La carcasse métallique se corrode activement, libérant des oxydes métalliques. Elle doit être triée en décharge."
    },
    {
        "id": 210,
        "descriptions": {
            "badly damaged": [
                "🔴 Un chariot élévateur manuel en acier lourd complètement rouillé, dont les fourches sont grippées.",
                "🔴 Cet équipement montre des roues en polyuréthane déchiquetées et un vérin hydraulique percé.",
                "🔴 Une carcasse de tire-palette encroûtée de sédiments calcaires durs, d'algues et de coquilles."
            ],
            "damaged": [
                "🟠 Un chariot élévateur présentant des déformations importantes sur ses fourches en acier lourd.",
                "🟠 Cet outil montre des plaques de rouille brune et un timon de guidage bloqué par la corrosion.",
                "🟠 Un équipement métallique déformé par des chocs répétés sous l'eau, couvert de vase."
            ],
            "worn": [
                "🟡 Un chariot élévateur manuel entier bien que sale, avec son mécanisme de levage de fourche mobile.",
                "🟡 Cet équipement de manutention a conservé son aspect général malgré des traces de rouille légères.",
                "🟡 Un tire-palette usagé jeté récemment dans le canal, reposant au fond de l'eau."
            ]
        },
        "fun_fact": "Le chariot élévateur manuel (ou transpalette) est un équipement de manutention lourd fabriqué en acier allié de forte épaisseur. Son poids important écrase le relief sédimentaire et détruit les frayères de poissons. De plus, son vérin contient de l'huile hydraulique hautement toxique (contenant des additifs hydrocarbures et des métaux) qui peut fuir dans l'eau en cas de corrosion du vérin, empoisonnant les poissons et la faune environnante. Il doit être valorisé en décharge."
    },
    {
        "id": 211,
        "descriptions": {
            "badly damaged": [
                "🔴 Une baignoire en acrylique complètement brisée en morceaux coupants jaunis par la lumière solaire.",
                "🔴 Cet équipement montre des parois en plastique rigide craquelées et de la mousse d'isolation pourrie.",
                "🔴 Des débris de plastique acrylique blanc encroûtés de calcaire et de vase noire de fond."
            ],
            "damaged": [
                "🟠 Une baignoire en acrylique présentant des fissures majeures sur ses rebords et des rayures profondes.",
                "🟠 Cet équipement sanitaire montre des trous de plomberie encrassés par de la vase argileuse grise.",
                "🟠 Une baignoire déformée par l'écrasement mécanique hydrographique, remplie de sédiments fins."
            ],
            "worn": [
                "🟡 Une baignoire en acrylique blanc entière bien que sale, reposant sur le flanc au fond.",
                "🟡 Cet équipement a conservé sa rigidité caractéristique malgré des éraflures superficielles.",
                "🟡 Une baignoire de salle de bain usagée jetée récemment, reposant près de la rive."
            ]
        },
        "fun_fact": "La baignoire en acrylique est constituée d'une feuille de polyméthacrylate de méthyle (PMMA) thermoformée, renforcée sur sa face arrière par de la fibre de verre imprégnée de résine polyester et de la mousse de polyuréthane. Ces polymères de synthèse sont hautement résistants à la biodégradation. Dans l'eau, ce déchet volumineux mettra des siècles à se détruire physiquement, se fragmentant en microparticules acryliques toxiques qui contaminent la faune marine. Il doit être déposé aux encombrants en décharge."
    },
    {
        "id": 212,
        "descriptions": {
            "badly damaged": [
                "🔴 Un lavabo en céramique complètement brisé en éclats de faïence pointus comme des rasoirs.",
                "🔴 Cet équipement sanitaire montre des morceaux de céramique blanche encroûtés de vase noire fétide.",
                "🔴 Des éclats de faïence émaillée cassée de lavabo, enfouis sous le gravier de fond de rivière."
            ],
            "damaged": [
                "🟠 Un lavabo en céramique présentant des fêlures majeures traversant toute sa structure blanche.",
                "🟠 Cet équipement montre des éclats d'émail sur ses rebords et des traces de calcaire épaisses.",
                "🟠 Un lavabo blanc déformé par l'écrasement hydrographique de fond, contenant du sable grossier."
            ],
            "worn": [
                "🟡 Un lavabo en céramique blanche entier bien que sale, reposant droit sur le lit de sable.",
                "🟡 Cet équipement de salle de bain a conservé son trou d'évacuation métallique malgré des éraflures.",
                "🟡 Un lavabo usagé jeté récemment dans le canal, reposant au fond près de la berge."
            ]
        },
        "fun_fact": "Le lavabo en céramique fendu est fabriqué en porcelaine sanitaire ou en grès émaillé (argile cuite à très haute température et vitrifiée). C'est un matériau inerte chimiquement qui ne libère pas de substances toxiques dans l'eau. Cependant, sa rupture produit des débris de céramique extrêmement tranchants et coupants qui représentent un danger mortel pour les mammifères aquatiques et les poissons benthiques, qui peuvent s'y blesser gravement. Il doit rejoindre la benne à gravats de la décharge."
    }
]

updates_en = [
    {
        "id": 203,
        "descriptions": {
            "badly damaged": [
                "🔴 A tumble dryer completely rusted and crushed, its metal body crumbling into iron oxide plates.",
                "🔴 This appliance shows a blocked and deformed stainless steel drum, filled with black silt and algae.",
                "🔴 A tumble dryer casing encrusted with hard calcareous marine sediments and shells."
            ],
            "damaged": [
                "🟠 A tumble dryer showing significant deformations on its sides and a broken glass door.",
                "🟠 This household appliance shows split plastic internal ducts and oxidized circuits.",
                "🟠 A tumble dryer deformed by mechanical crushing under water, containing sand and silt."
            ],
            "worn": [
                "🟡 A tumble dryer whole although dirty, its steel sheet body showing no cracks.",
                "🟡 This appliance has kept its power cord at the back despite superficial surface wear.",
                "🟡 A used tumble dryer discarded recently, resting on its side at the bottom of the canal."
            ]
        },
        "fun_fact": "The blocked drum tumble dryer is part of the large household appliances (LHA) rejected in the environment. It contains a significant amount of steel (body and drum), a heavy rotation motor made of copper and steel, electronic control components, as well as pipes and tubs made of PP or ABS plastic. In waterways, its presence disrupts sedimentation and destroys the local biotope by crushing benthic flora. The slow corrosion of metals releases harmful heavy metals. It must be deposited at the landfill."
    },
    {
        "id": 204,
        "descriptions": {
            "badly damaged": [
                "🔴 A electric water heater completely disemboweled and rusted, its inner insulation peeling off.",
                "🔴 This huge tank shows metal walls pierced by corrosion and a cylinder full of black mud.",
                "🔴 A water heater casing encrusted with hard sediments and decaying filamentous algae."
            ],
            "damaged": [
                "🟠 A water heater showing major dents on its sheet metal shell and bent copper pipes.",
                "🟠 This electric cylinder shows pronounced red rust marks and a broken safety valve.",
                "🟠 A water tank deformed by violent bottom currents, containing sand and fine silt."
            ],
            "worn": [
                "🟡 A cylindrical electric water heater complete although dirty, showing superficial scratches.",
                "🟡 This appliance has kept its inlet and outlet threaded connections despite light rust marks.",
                "🟡 A used water heater discarded recently, resting flat near a bed of pebbles."
            ]
        },
        "fun_fact": "The electric water heater (or cylinder) consists of an outer shell of painted steel sheet, a thick layer of polyurethane foam insulation, an enameled steel inner tank, a magnesium or titanium anode, and a shielded heating resistor. The polyurethane foam degrades in water, releasing toxic volatile organic compounds and persistent microplastics. Resistor corrosion releases copper and nickel, which are toxic to aquatic life. It must be recycled via the landfill circuit."
    },
    {
        "id": 205,
        "descriptions": {
            "badly damaged": [
                "🔴 A television whose flat screen is completely shattered into fine and sharp glass shards.",
                "🔴 This device shows a crushed plastic frame and eroded, green-oxidized circuit boards.",
                "🔴 Plastic and glass debris from a broken television, lying in the black clayey silt."
            ],
            "damaged": [
                "🟠 A flat screen television showing major cracks on its hard plastic casing and scratches.",
                "🟠 This electronic device shows a shattered panel and gray internal moisture traces.",
                "🟠 A television deformed by violent impacts against rocks, containing fine sand."
            ],
            "worn": [
                "🟡 A flat screen television whole although dirty, resting flat at the bottom of the water.",
                "🟡 This device has kept its original remote control wedged in its base despite signs of wear.",
                "🟡 A used television discarded recently, resting near a sandy bank."
            ]
        },
        "fun_fact": "The broken flat screen television is a particularly complex waste electrical and electronic equipment (WEEE). It features an LCD panel containing liquid crystals (harmful fluorinated organic compounds) or OLED diodes, an LED backlight, PMMA optical diffusers, an electronic board containing lead, cadmium, and arsenic, and a flame-retardant plastic casing. Immersion of these components contaminates water and poisons benthic fauna for a long time. It must be recycled at the landfill."
    },
    {
        "id": 206,
        "descriptions": {
            "badly damaged": [
                "🔴 A computer tower whose metal body is completely rusted and crushed.",
                "🔴 This computer shows internal motherboards and hard drives eaten away by water oxidation.",
                "🔴 Plastic, steel, and PC integrated circuit component debris, encrusted with black mud."
            ],
            "damaged": [
                "🟠 A computer tower showing bent hinges and metal panels.",
                "🟠 This casing shows deep abrasive sand scratches and a broken disc drive.",
                "🟠 A PC case deformed by mechanical crushing, filled with fine clayey silt."
            ],
            "worn": [
                "🟡 A computer tower whole although dirty, with its external USB ports still regular.",
                "🟡 This casing has kept its internal fans despite superficial signs of wear.",
                "🟡 A used computer discarded recently in the canal, resting at the bottom near the bank."
            ]
        },
        "fun_fact": "The computer tower is heavy electronic waste (WEEE) that contains a wide variety of materials: galvanized steel (chassis), ABS plastic, printed circuit boards made of epoxy resin and fiberglass loaded with heavy metals (lead in solder, cadmium, chromium) and precious metals (gold, silver, copper). In water, metals slowly oxidize and dissolve as highly toxic metal ions that accumulate in river sediments. It must be sorted in the WEEE bin at the landfill."
    },
    {
        "id": 207,
        "descriptions": {
            "badly damaged": [
                "🔴 An inkjet printer completely crushed and broken, its ink cartridges split open.",
                "🔴 This device shows printed circuits green from oxidation and broken feed rollers.",
                "🔴 Rigid plastic and empty inkjet cartridge debris, lying in the clayey silt."
            ],
            "damaged": [
                "🟠 A printer showing major cracks on its plastic body and broken hinges.",
                "🟠 This office device shows a cracked scanner glass and visible black ink runs.",
                "🟠 A printer deformed by bottom pressure, containing fine sand and algae."
            ],
            "worn": [
                "🟡 An inkjet printer whole although dirty, with its scanner glass uncracked.",
                "🟡 This office device has kept its original plastic buttons despite scratches.",
                "🟡 A used printer discarded recently in the canal, resting near a stone."
            ]
        },
        "fun_fact": "The inkjet printer contains engineering plastics (ABS, polystyrene), electric motors, steel shafts, electronic boards, and liquid ink cartridges. Inkjet ink contains organic dyes or metallic pigments, solvents (glycols), and toxic preservatives. If the printer is submerged, these inks dissolve instantly, creating a toxic slick that poisons filtering organisms and local fish. It must join the sorting bin at the landfill."
    },
    {
        "id": 208,
        "descriptions": {
            "badly damaged": [
                "🔴 A brick garden pizza oven completely collapsed into a pile of rubble and mortar.",
                "🔴 This oven shows a broken refractory stone hearth and a completely rusted steel door.",
                "🔴 Baked refractory brick and charcoal-black soot debris, buried in the mud."
            ],
            "damaged": [
                "🟠 A pizza oven showing significant cracks on its brick dome and crumbled mortar.",
                "🟠 This equipment shows pronounced soot marks on its walls and a deformed structure.",
                "🟠 A garden oven deformed by mechanical crushing under water, containing sand and silt."
            ],
            "worn": [
                "🟡 A brick garden pizza oven whole although dirty, with its dome still stable.",
                "🟡 This equipment has kept its steel door despite superficial rust marks.",
                "🟡 A garden oven discarded recently, resting at the bottom near the canal bank."
            ]
        },
        "fun_fact": "The garden pizza oven is composed of refractory baked clay bricks, cement or lime mortar, quartz sand, rock wool (for insulation), and metallic structures made of steel or cast iron. In water, the mineral materials (bricks, mortar) are relatively inert and do not chemically pollute, but their collapse physically destroys the bottom biotope by burying benthic fauna under a pile of heavy rubble. It must be sorted in the rubble bin at the landfill."
    },
    {
        "id": 209,
        "descriptions": {
            "badly damaged": [
                "🔴 A lawnmower frame completely rusted and twisted, its combustion engine split open.",
                "🔴 This gardening device shows broken plastic wheels and a pierced metal deck.",
                "🔴 A lawnmower frame encrusted with black silt, lime, and marine shells."
            ],
            "damaged": [
                "🟠 A lawnmower frame showing deformations on its rusted steel deck.",
                "🟠 This device shows a metal blade bent by violent impacts and a broken handlebar.",
                "🟠 A mower deformed by bottom crushing, containing fine silt and sand."
            ],
            "worn": [
                "🟡 A lawnmower frame whole although dirty, with its wheels still movable.",
                "🟡 This device has kept its plastic gas tank despite superficial scratches.",
                "🟡 A used mower discarded recently in the canal, resting at the bottom of the clear water."
            ]
        },
        "fun_fact": "The lawnmower frame includes a steel or aluminum deck, plastic wheels, and a four-stroke gasoline engine. If it still contains gasoline or engine oil, its immersion is catastrophic for the aquatic ecosystem: hydrocarbons form an impermeable film on the surface blocking atmospheric oxygen and suffocating fish. The metal frame actively corrodes, releasing metal oxides. It must be sorted and recycled at the landfill."
    },
    {
        "id": 210,
        "descriptions": {
            "badly damaged": [
                "🔴 A manual fork lift in heavy steel completely rusted, its forks seized.",
                "🔴 This equipment shows shredded polyurethane wheels and a pierced hydraulic cylinder.",
                "🔴 A pallet jack frame encrusted with hard calcareous sediments, algae, and shells."
            ],
            "damaged": [
                "🟠 A fork lift showing significant deformations on its heavy steel forks.",
                "🟠 This tool shows brown rust plates and a steering handle blocked by corrosion.",
                "🟠 Metallic equipment deformed by repeated impacts underwater, covered in silt."
            ],
            "worn": [
                "🟡 A manual fork lift whole although dirty, with its fork lifting mechanism movable.",
                "🟡 This handling equipment has kept its general appearance despite light rust marks.",
                "🟡 A used pallet jack discarded recently in the canal, resting at the bottom of the water."
            ]
        },
        "fun_fact": "The manual forklift (or pallet jack) is heavy handling equipment made of very thick alloy steel. Its significant weight crushes bottom sediments and destroys fish spawning grounds. Furthermore, its cylinder contains highly toxic hydraulic oil (containing hydrocarbon additives and metals) which can leak into the water if the cylinder corrodes, poisoning fish and surrounding wildlife. It must be recovered in the scrap metal bin at the landfill."
    },
    {
        "id": 211,
        "descriptions": {
            "badly damaged": [
                "🔴 An acrylic bathtub completely broken into sharp pieces yellowed by sunlight.",
                "🔴 This equipment shows cracked rigid plastic walls and rotted insulation foam.",
                "🔴 White acrylic plastic debris encrusted with lime and black bottom silt."
            ],
            "damaged": [
                "🟠 An acrylic bathtub showing major cracks on its edges and deep scratches.",
                "🟠 This sanitary equipment shows plumbing holes clogged with gray clayey silt.",
                "🟠 A bathtub deformed by hydrographic mechanical crushing, filled with fine sediments."
            ],
            "worn": [
                "🟡 A white acrylic bathtub whole although dirty, resting on its side at the bottom.",
                "🟡 This equipment has kept its characteristic rigidity despite superficial scratches.",
                "🟡 A used bathroom bathtub discarded recently, resting near the bank."
            ]
        },
        "fun_fact": "The acrylic bathtub consists of a thermoformed polymethyl methacrylate (PMMA) sheet, reinforced on its back with fiberglass impregnated with polyester resin and polyurethane foam. These synthetic polymers are highly resistant to biological degradation. In water, this bulky waste will take centuries to physically destroy itself, fragmenting into toxic acrylic microparticles that contaminate marine life. It must be deposited at the landfill."
    },
    {
        "id": 212,
        "descriptions": {
            "badly damaged": [
                "🔴 A ceramic sink completely broken into razor-sharp shards of pottery.",
                "🔴 This sanitary equipment shows white ceramic pieces encrusted with foul black silt.",
                "🔴 Broken enameled ceramic shards from a sink, buried under river bottom gravel."
            ],
            "damaged": [
                "🟠 A ceramic sink showing major cracks running through its white structure.",
                "🟠 This equipment shows enamel chips on its edges and thick lime traces.",
                "🟠 A white sink deformed by hydrographic bottom crushing, containing coarse sand."
            ],
            "worn": [
                "🟡 A white ceramic sink whole although dirty, resting straight on the sand bed.",
                "🟡 This bathroom equipment has kept its metallic drain hole despite scratches.",
                "🟡 A used sink discarded recently in the canal, resting at the bottom near the bank."
            ]
        },
        "fun_fact": "The cracked ceramic sink is made of sanitary porcelain or enameled stoneware (clay baked at very high temperatures and vitrified). It is a chemically inert material that does not release toxic substances into water. However, its breakage produces extremely sharp ceramic debris that represents a fatal danger of injury to aquatic mammals and benthic fish, which can cut themselves severely. It must join the rubble bin at the landfill."
    }
]

apply_updates(updates_fr, updates_en)
print("Batch 7 updated successfully!")
