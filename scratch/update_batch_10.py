# -*- coding: utf-8 -*-
from update_junk_util import apply_updates

updates_fr = [
    {
        "id": 233,
        "descriptions": {
            "badly damaged": [
                "🔴 Un module lunaire factice dont les panneaux dorés sont complètement déchirés, la structure métallique tordue et pleine de vase.",
                "🔴 Cette réplique montre des montants en aluminium rompus et une carrosserie encroûtée d'algues et de calcaire épais.",
                "🔴 Des débris de feuille d'or synthétique et de tôles froissées de module spatial, à moitié enfouis au fond."
            ],
            "damaged": [
                "🟠 Une réplique de module lunaire présentant des parois enfoncées et des antennes cassées par le ressac.",
                "🟠 Cet équipement d'exposition montre des traces d'oxydation blanche sur ses structures en alliage léger.",
                "🟠 Un module spatial factice déformé par l'écrasement mécanique, rempli d'eau stagnante et de sable."
            ],
            "worn": [
                "🟡 Une réplique de module lunaire entière bien que sale, reposant droite sur le lit du cours d'eau.",
                "🟡 Cet engin a conservé ses quatre pieds d'atterrissage caractéristiques malgré des éraflures superficielles.",
                "🟡 Une réplique de module spatial usagée perdue lors d'un transport, reposant au fond près de la berge."
            ]
        },
        "fun_fact": "Le module lunaire (réplique) est un déchet insolite et de grande taille, construit principalement en alliages d'aluminium légers et recouvert d'un film isolant multicouche doré (Kapton). Bien que ce soit une réplique d'exposition et non un véritable engin spatial, ses dimensions imposantes modifient localement le lit de la rivière et perturbent le flux hydraulique, créant des remous et des zones de dépôt de vase. Le film plastique doré se fragmente sous l'eau en libérant des milliers de paillettes plastiques brillantes qui polluent le biotope. Il doit être évacué en décharge."
    },
    {
        "id": 234,
        "descriptions": {
            "badly damaged": [
                "🔴 Une carcasse de char d'assaut dont le blindage en acier est percé par la rouille, la tourelle arrachée et pleine de boue.",
                "🔴 Ce blindé montre des chenilles brisées et disjointes sous l'action corrosive marine, les galets de roulement grippés.",
                "🔴 Une structure massive de tank encroûtée de sédiments marins calcaires durs, d'algues filamenteuses et de moules."
            ],
            "damaged": [
                "🟠 Un char d'assaut présentant des déformations importantes sur sa coque blindée et des plaques d'acier fêlées.",
                "🟠 Ce véhicule militaire montre des traces prononcées d'oxydation rousse et un canon tordu sous l'eau.",
                "🟠 Un tank déformé par des courants de fond ou des explosions passées, contenant du sable et du limon argileux."
            ],
            "worn": [
                "🟡 Une carcasse de char d'assaut en acier épais entière bien que très sale, avec son canon pointé vers l'avant.",
                "🟡 Ce véhicule militaire a conservé ses plaques de blindage intactes malgré de larges traces d'oxydation de surface.",
                "🟡 Un tank usagé abandonné après des exercices militaires passés, reposant à plat sur le fond sableux."
            ]
        },
        "fun_fact": "La carcasse de char d'assaut est un déchet militaire colossal composé de blindage en acier allié ultra-épais (contenant du manganèse, du chrome ou du nickel). Son poids de plusieurs dizaines de tonnes écrase définitivement le relief sédimentaire sous-jacent et détruit toute forme de vie benthique à son point d'impact. Bien que sa corrosion prenne plusieurs siècles, elle libère de grandes quantités d'oxydes métalliques et de petits résidus de graisses lourdes ou d'huiles hydrauliques toxiques. Son extraction nécessite des moyens de relevage militaires lourds en décharge."
    },
    {
        "id": 235,
        "descriptions": {
            "badly damaged": [
                "🔴 Une tourelle de cuirassé en acier massif complètement rouillée et éventrée, les canons brisés et grippés.",
                "🔴 Ce bloc de blindage s'effrite en plaques d'oxyde de fer épaisses sous l'action corrosive saline.",
                "🔴 Une structure de tourelle d'artillerie encroûtée de vase noire, d'algues géantes et de colonies de moules."
            ],
            "damaged": [
                "🟠 Une tourelle de navire présentant de larges fêlures sur ses plaques de blindage et des canons tordus.",
                "🟠 Cet équipement militaire montre des traces de corrosion rousse profondes et des trappes d'accès bloquées.",
                "🟠 Une tourelle d'artillerie déformée par le ressac ou un naufrage, contenant du sable et du limon fin."
            ],
            "worn": [
                "🟡 Une tourelle de cuirassé entière bien que sale et couverte de rouille superficielle, ses canons pointés au loin.",
                "🟡 Ce bloc d'acier de marine a conservé ses lignes géométriques malgré des éraflures profondes de surface.",
                "🟡 Une tourelle militaire perdue lors d'un naufrage historique, reposant à plat sur le lit sablonneux de la mer."
            ]
        },
        "fun_fact": "La tourelle de cuirassé est une pièce d'artillerie navale colossale pesant des centaines de tonnes de blindage d'acier Krupp ou similaire. Son immersion modifie de façon permanente la topographie marine et perturbe les courants locaux. En s'oxydant très lentement à cause du sel, elle rejette d'importantes quantités d'oxydes de fer qui modifient la chimie des sédiments locaux. Bien que ces oxydes soient peu toxiques, son extraction est impossible sans navires de relevage de très grande capacité. Elle doit être laissée en place ou démantelée en décharge."
    },
    {
        "id": 236,
        "descriptions": {
            "badly damaged": [
                "🔴 Une mine magnétique complètement rouillée et percée, dont le mécanisme de mise à feu est détruit et plein de vase.",
                "🔴 Cet engin explosif montre des parois en acier s'effritant au toucher et des aimants de fixation désolidarisés.",
                "🔴 Une enveloppe de mine encroûtée de sédiments marins calcaires durs, d'algues filamenteuses et de coquillages."
            ],
            "damaged": [
                "🟠 Une mine magnétique présentant des enfoncements majeurs et des traces d'oxydation rousse profondes.",
                "🟠 Cet appareil militaire montre des fissures sur sa coque et des composants magnétiques de fixation bloqués.",
                "🟠 Une mine déformée par des chocs violents sous l'eau, contenant du sable grossier et de la vase noire."
            ],
            "worn": [
                "🟡 Une mine magnétique inerte entière bien que sale, présentant ses fixations magnétiques caractéristiques.",
                "🟡 Cet engin a conservé sa forme sphérique ou cylindrique malgré de légères traces de rouille de surface.",
                "🟡 Une mine militaire usagée datant d'un conflit passé, reposant au fond près d'une épave métallique."
            ]
        },
        "fun_fact": "La mine magnétique (WW2) est un déchet militaire historique dangereux contenant de l'acier épais et, s'il est actif, des charges explosives (TNT, RDX) et des amorces chimiques. Même inerte, sa corrosion lente libère du fer et des métaux d'aimant (nickel, cobalt), mais s'il s'agit d'une munition non explosée, elle présente un risque d'explosion majeur et de pollution chimique par les composés nitrés toxiques de l'explosif. Sa manipulation requiert l'intervention immédiate d'une équipe de démineurs de la marine nationale."
    },
    {
        "id": 237,
        "descriptions": {
            "badly damaged": [
                "🔴 Un satellite de télécom brisé, les panneaux solaires en silicium en miettes et la structure en aluminium tordue.",
                "🔴 Cet appareil spatial montre des câbles dorés arrachés et des circuits électroniques rongés par l'oxydation de l'eau.",
                "🔴 Des débris de plastique, de réflecteurs et de composants électroniques de satellite, encroûtés de boue noire."
            ],
            "damaged": [
                "🟠 Un satellite présentant des fissures majeures sur son corps principal et des antennes paraboliques tordues.",
                "🟠 Ce boîtier montre des rayures profondes et des décolorations prononcées de ses isolants thermiques dorés.",
                "🟠 Un satellite déformé par son crash et le ressac de l'eau, contenant de la vase fine et du sable."
            ],
            "worn": [
                "🟡 Un satellite de télécom entier bien que très sale, avec ses réflecteurs paraboliques encore identifiables.",
                "🟡 Cet équipement a conservé ses liaisons mécaniques principales malgré de légères traces d'oxydation de surface.",
                "🟡 Un satellite tombé de l'espace jeté récemment dans l'océan, reposant au fond près d'une fosse de mer."
            ]
        },
        "fun_fact": "Le satellite de télécom HS est un déchet spatial technologique composé de matériaux de pointe : fibres de carbone composites, alliages de titane et d'aluminium aéronautique, silicium de qualité solaire et connectiques plaquées or et argent. S'il survit à sa rentrée atmosphérique et finit au fond de l'eau, sa dégradation lente expose le biotope à des métaux lourds et des résidus de composants électroniques (arsenic, plomb, béryllium) très toxiques et à longue persistance environnementale. Il doit être évacué en décharge."
    },
    {
        "id": 238,
        "descriptions": {
            "badly damaged": [
                "🔴 Un fragment de station spatiale en titane tordu et déchiré, recouvert d'un limon fétide et d'algues sombres.",
                "🔴 Ce reste de station Mir montre des conduites internes brisées et une structure encroûtée de sédiments durs.",
                "🔴 Des débris de plastique isolant et de tôles de station spatiale brisées, à moitié enfouis sous le gravier."
            ],
            "damaged": [
                "🟠 Un fragment de station Mir présentant des enfoncements majeurs et des panneaux métalliques fêlés.",
                "🟠 Cette pièce aéronautique montre des traces d'oxydation blanche de surface et des câblages coupés.",
                "🟠 Une structure métallique déformée par l'écrasement mécanique hydrographique, remplie de sédiments fins."
            ],
            "worn": [
                "🟡 Un fragment de station Mir entier bien que très sale, avec son logo d'identification encore lisible.",
                "🟡 Cette structure aérospatiale a conservé ses hublots en verre renforcé intacts malgré des éraflures.",
                "🟡 Un débris de station spatiale usagé tombé récemment dans le fleuve, reposant à plat sur le lit sableux."
            ]
        },
        "fun_fact": "Le fragment de station Mir provient du démantèlement contrôlé ou de la chute de la célèbre station orbitale russe en 2001. Composé d'alliages d'aluminium de haute performance, de titane et de couches d'isolants en polyimide, ce déchet présente une résistance chimique exceptionnelle. Dans l'eau, il ne se décompose pas et modifie de façon permanente la morphologie locale du lit du fleuve ou de la mer, constituant un obstacle pour la faune. Il s'agit d'un déchet d'exception qui doit faire l'objet d'un relevage lourd."
    },
    {
        "id": 239,
        "descriptions": {
            "badly damaged": [
                "🔴 Une capsule spatiale dont le bouclier thermique est complètement effrité et troué, l'habitacle plein de boue noire.",
                "🔴 Cet engin spatial montre des hublots en verre trempé brisés et des parois d'aluminium rongées par le sel.",
                "🔴 Une carcasse de capsule Apollo encroûtée de sédiments marins calcaires durs, d'algues et de coquilles."
            ],
            "damaged": [
                "🟠 Une capsule spatiale présentant des déformations importantes sur sa coque externe et des écoutilles fêlées.",
                "🟠 Cet appareil aéronautique montre des traces de brûlure d'entrée atmosphérique et des métaux rayés.",
                "🟠 Une capsule déformée par un amerrissage violent et la pression de l'eau, contenant du sable grossier."
            ],
            "worn": [
                "🟡 Une capsule spatiale Apollo entière bien que sale, avec sa structure conique d'origine bien identifiable.",
                "🟡 Cet engin a conservé ses anneaux de suspension et ses fixations métalliques malgré de légères éraflures.",
                "🟡 Une capsule spatiale usagée récupérée puis perdue lors d'un transport, reposant au fond de l'eau."
            ]
        },
        "fun_fact": "La capsule spatiale Apollo est un déchet aérospatial historique composé de structures en nid d'abeille d'aluminium et d'acier inoxydable, isolées par des résines époxy et de la fibre de verre, avec un bouclier thermique en résine phénolique. Si elle finit dans l'eau, le bouclier thermique se dégrade lentement en libérant des composés phénoliques toxiques. La structure métallique résiste très bien, tandis que le volume de la capsule perturbe fortement le courant d'eau. Son relevage relève de la décharge lourde."
    },
    {
        "id": 240,
        "descriptions": {
            "badly damaged": [
                "🔴 Un rover martien dont le châssis en titane est brisé en deux, les roues métalliques tordues et bloquées.",
                "🔴 Cet appareil montre des caméras optiques brisées et des puces en silicium rongées couvertes de vase noire.",
                "🔴 Des débris de plastique et de circuits imprimés de rover martien, gisant dans le limon argileux de fond."
            ],
            "damaged": [
                "🟠 Un prototype de rover martien présentant des fissures importantes sur son bras mécanique et sa coque.",
                "🟠 Cet appareil technologique montre un panneau solaire brisé et des engrenages de direction encrassés.",
                "🟠 Un rover déformé par un choc mécanique violent sous l'eau, contenant du sable et du limon fin."
            ],
            "worn": [
                "🟡 Un rover martien de démonstration entier bien que très sale, avec ses six roues encore en place.",
                "🟡 Cet appareil a conservé sa caméra panoramique métallique malgré de légères traces d'oxydation.",
                "🟡 Un rover martien usagé jeté après des tests secrets en milieu aquatique, reposant près de la berge."
            ]
        },
        "fun_fact": "Le rover martien (prototype) est un déchet technologique d'exception. Fabriqué en titane, fibre de carbone composite et doté de batteries rechargeables hautes performances et d'équipements scientifiques de pointe (spectromètres, lasers), son immersion est préoccupante. Les composants électroniques contiennent des semi-conducteurs toxiques et sa batterie peut libérer des solvants fluorés nocifs. Ce déchet hors normes doit être extrait avec le plus grand soin par des spécialistes pour rejoindre la décharge."
    },
    {
        "id": 241,
        "descriptions": {
            "badly damaged": [
                "🔴 Un cœur de réacteur inerte en plomb et acier lourd complètement Eventré par la corrosion et rouillé.",
                "🔴 Ce conteneur massif montre des parois de plomb s'effritant et un intérieur rempli de boue noire toxique.",
                "🔴 Une carcasse de réacteur inerte encroûtée de sédiments marins calcaires durs et d'algues filamenteuses."
            ],
            "damaged": [
                "🟠 Un cœur de réacteur présentant des fêlures majeures sur ses parois épaisses et des vannes de sécurité tordues.",
                "🟠 Ce conteneur montre des plaques de rouille rousse et des traces d'oxydation grise sur ses blindages de plomb.",
                "🟠 Un bloc métallique déformé par des forces de fond extrêmes, contenant du sable et du limon fétide."
            ],
            "worn": [
                "🟡 Un cœur de réacteur inerte en acier et plomb entier, bien que sale et présentant de légères traces d'usure.",
                "🟡 Ce conteneur a conservé ses poignées de manutention lourdes malgré des éraflures superficielles.",
                "🟡 Un cœur de réacteur usagé inerte jeté secrètement, reposant à plat sur le lit de sable fin."
            ]
        },
        "fun_fact": "Le cœur de réacteur inerte est constitué d'une enveloppe en acier inoxydable de forte épaisseur, blindée par des plaques de plomb de protection contre les radiations. Bien que le combustible nucléaire ait été retiré (rendant le cœur inerte), la présence de tonnes de plomb représente un risque écologique colossal. Le plomb est un métal lourd hautement toxique qui ne se dégrade pas et provoque le saturnisme chez les poissons, bloquant leur reproduction. Son extraction militaire est urgente en décharge."
    },
    {
        "id": 242,
        "descriptions": {
            "badly damaged": [
                "🔴 Une plaque de sonde Pioneer dorée complètement pliée en deux, la surface gravée rayée et illisible.",
                "🔴 Ce panneau d'aluminium montre des bords déchirés et une surface encroûtée d'algues et de vase noire.",
                "🔴 Des éclats dorés d'aluminium de plaque spatiale, à moitié enfouis sous le gravier de fond de rivière."
            ],
            "damaged": [
                "🟠 Une plaque spatiale en aluminium présentant des déformations sur sa surface réfléchissante.",
                "🟠 Cette plaque dorée montre des rayures de frottement de sable qui effacent partiellement les gravures d'humains.",
                "🟠 Une plaque de sonde spatiale déformée par le courant, contenant du limon fin sur ses bordures."
            ],
            "worn": [
                "🟡 Une plaque dorée de la sonde Pioneer entière bien que très sale, les schémas gravés encore visibles.",
                "🟡 Cette plaque en aluminium a conservé son brillant jaune d'or d'origine malgré de légères traces d'usure.",
                "🟡 Une réplique ou un fragment de plaque spatiale perdue récemment, reposant sur un lit de galets clairs."
            ]
        },
        "fun_fact": "La plaque de la sonde Pioneer est un document métallique historique en aluminium anodisé recouvert d'une fine couche d'or. Elle porte un message graphique de l'humanité destiné à d'éventuelles intelligences extraterrestres. Si elle se retrouve dans l'eau douce ou salée, l'or protège parfaitement l'aluminium sous-jacent de toute corrosion chimique active. C'est un artefact culturel d'une valeur symbolique inestimable qui doit être récupéré et valorisé dans le circuit de la décharge."
    }
]

updates_en = [
    {
        "id": 233,
        "descriptions": {
            "badly damaged": [
                "🔴 A mock lunar module whose gold panels are completely torn, the metal structure bent and full of silt.",
                "🔴 This replica shows broken aluminum struts and a casing encrusted with algae and thick lime.",
                "🔴 Synthetic gold foil and crumpled space module sheet metal debris, half-buried at the bottom."
            ],
            "damaged": [
                "🟠 A lunar module replica showing caved-in walls and antennas broken by the surf.",
                "🟠 This exhibition equipment shows white oxidation marks on its lightweight alloy structures.",
                "🟠 A mock space module deformed by mechanical crushing, filled with stagnant water and sand."
            ],
            "worn": [
                "🟡 A brick-built lunar module replica whole although dirty, resting straight on the waterway bed.",
                "🟡 This device has kept its four characteristic landing legs despite superficial scratches.",
                "🟡 A used space module replica lost during transport, resting at the bottom near the bank."
            ]
        },
        "fun_fact": "The lunar module (replica) is an unusual and large waste item, mainly constructed from lightweight aluminum alloys and covered with a golden multi-layer insulating film (Kapton). Although it is an exhibition replica and not a real space vehicle, its imposing size locally modifies the riverbed and disrupts the hydraulic flow, creating eddies and silt deposition zones. The gold plastic film fragments under water, releasing thousands of shiny plastic flakes that pollute the biotope. It must be evacuated to the landfill."
    },
    {
        "id": 234,
        "descriptions": {
            "badly damaged": [
                "🔴 A tank chassis whose steel armor is pierced by rust, the turret torn off and full of mud.",
                "🔴 This armored vehicle shows track links broken and disjointed under marine corrosion, road wheels seized.",
                "🔴 A massive tank structure encrusted with hard marine sediments, filamentous algae, and mussels."
            ],
            "damaged": [
                "🟠 An battle tank showing significant deformations on its armored hull and cracked steel plates.",
                "🟠 This military vehicle shows pronounced red oxidation marks and a barrel bent under water.",
                "🟠 A tank deformed by bottom currents or past explosions, containing sand and clayey silt."
            ],
            "worn": [
                "🟡 A heavy steel armored tank chassis whole although very dirty, with its barrel pointed forward.",
                "🟡 This military vehicle has kept its armor plates intact despite wide surface oxidation marks.",
                "🟡 A used tank abandoned after past military exercises, resting flat on the sandy bottom."
            ]
        },
        "fun_fact": "The tank chassis is colossal military waste composed of ultra-thick alloy steel armor (containing manganese, chromium, or nickel). Its weight of several dozen tons permanently crushes the underlying sediment relief and destroys all forms of benthic life at its point of impact. Although its corrosion takes several centuries, it releases large quantities of metal oxides and small residues of heavy greases or toxic hydraulic oils. Its extraction requires heavy military lifting equipment to the landfill."
    },
    {
        "id": 235,
        "descriptions": {
            "badly damaged": [
                "🔴 A battleship turret in massive steel completely rusted and disemboweled, barrels broken and seized.",
                "🔴 This armor block crumbles into thick iron oxide plates under saline corrosive action.",
                "🔴 An artillery turret structure encrusted with black silt, giant algae, and mussel colonies."
            ],
            "damaged": [
                "🟠 A naval ship turret showing wide cracks on its armor plates and bent barrels.",
                "🟠 This military equipment shows deep red corrosion marks and blocked access hatches.",
                "🟠 An artillery turret deformed by the backwash or a shipwreck, containing sand and fine silt."
            ],
            "worn": [
                "🟡 A battleship turret whole although dirty and covered in superficial rust, its barrels pointed far away.",
                "🟡 This marine steel block has kept its geometric lines despite deep surface scratches.",
                "🟡 A military turret lost during a historical shipwreck, resting flat on the sandy bed of the sea."
            ]
        },
        "fun_fact": "The battleship turret is a colossal naval artillery piece weighing hundreds of tons of Krupp steel armor or similar. Its immersion permanently modifies the marine topography and disrupts local currents. By oxidizing very slowly due to salt, it rejects large quantities of iron oxides that alter the chemistry of local sediments. Although these oxides are not very toxic, its extraction is impossible without very large capacity lifting vessels. It must be left in place or dismantled in the landfill."
    },
    {
        "id": 236,
        "descriptions": {
            "badly damaged": [
                "🔴 A magnetic mine completely rusted and pierced, its firing mechanism destroyed and full of silt.",
                "🔴 This explosive device shows steel walls crumbling to the touch and detached attachment magnets.",
                "🔴 A mine casing encrusted with hard marine sediments, filamentous algae, and shells."
            ],
            "damaged": [
                "🟠 A magnetic mine showing major dents and deep red oxidation marks.",
                "🟠 This military device shows cracks on its casing and blocked magnetic attachment components.",
                "🟠 A mine deformed by violent impacts under water, containing coarse sand and black silt."
            ],
            "worn": [
                "🟡 An inert magnetic mine whole although dirty, showing its characteristic magnetic mounts.",
                "🟡 This device has kept its spherical or cylindrical shape despite light surface rust marks.",
                "🟡 A used military mine from a past conflict, resting at the bottom near a metal wreck."
            ]
        },
        "fun_fact": "The magnetic mine (WW2) is dangerous historical military waste containing thick steel and, if active, explosive charges (TNT, RDX) and chemical primers. Even if inert, its slow corrosion releases iron and magnet metals (nickel, cobalt), but if it is unexploded ordnance, it presents a major explosion hazard and chemical pollution from the toxic nitrogen compounds of the explosive. Its handling requires the immediate intervention of a Navy bomb disposal team. It must go to the landfill."
    },
    {
        "id": 237,
        "descriptions": {
            "badly damaged": [
                "🔴 A telecom satellite broken, solar silicon panels in crumbs and bent aluminum structure.",
                "🔴 This space device shows torn gold cables and electronic circuits eaten by water oxidation.",
                "🔴 Plastic, reflector, and electronic component fragments from a satellite, encrusted with black mud."
            ],
            "damaged": [
                "🟠 A satellite showing major cracks on its main body and bent satellite dish antennas.",
                "🟠 This casing shows deep scratches and pronounced discoloration of its golden thermal insulators.",
                "🟠 A satellite deformed by its crash and water backwash, containing fine silt and sand."
            ],
            "worn": [
                "🟡 A telecom satellite whole although very dirty, with its parabolic reflectors still identifiable.",
                "🟡 This equipment has kept its main mechanical connections despite light surface oxidation marks.",
                "🟡 A satellite fallen from space recently thrown into the ocean, resting at the bottom near a sea trench."
            ]
        },
        "fun_fact": "The broken telecom satellite is technological space waste composed of advanced materials: composite carbon fibers, titanium and aerospace aluminum alloys, solar-grade silicon, and gold- and silver-plated connectors. If it survives atmospheric reentry and ends up at the bottom of the water, its slow degradation exposes the biotope to heavy metals and electronic components residues (arsenic, lead, beryllium) that are highly toxic and have long environmental persistence. It must go to the landfill."
    },
    {
        "id": 238,
        "descriptions": {
            "badly damaged": [
                "🔴 A space station fragment in titanium bent and torn, covered in foul silt and dark algae.",
                "🔴 This Mir station remnant shows broken internal pipes and a structure encrusted with hard sediments.",
                "🔴 Insulating plastic and broken space station sheet metal debris, half-buried under the gravel."
            ],
            "damaged": [
                "🟠 A Mir station fragment showing major dents and cracked metal panels.",
                "🟠 This aeronautical part shows white surface oxidation marks and cut wiring.",
                "🟠 A metallic structure deformed by hydrographic mechanical crushing, filled with fine sediments."
            ],
            "worn": [
                "🟡 A Mir station fragment whole although very dirty, with its identification logo still readable.",
                "🟡 This aerospace structure has kept its reinforced glass windows intact despite scratches.",
                "🟡 Used space station debris fallen recently into the river, resting flat on the sandy bed."
            ]
        },
        "fun_fact": "The Mir station fragment comes from the controlled deorbiting or fall of the famous Russian orbital station in 2001. Composed of high-performance aluminum alloys, titanium, and polyimide insulation layers, this waste shows exceptional chemical resistance. In water, it does not decompose and permanently modifies the local morphology of the riverbed or sea, constituting an obstacle for fauna. This is exceptional waste that requires heavy lifting to be sent to the landfill."
    },
    {
        "id": 239,
        "descriptions": {
            "badly damaged": [
                "🔴 A space capsule whose heat shield is completely crumbled and holed, the cabin full of black mud.",
                "🔴 This space vehicle shows broken tempered glass windows and aluminum walls eaten away by salt.",
                "🔴 An Apollo capsule casing encrusted with hard marine sediments, algae, and shells."
            ],
            "damaged": [
                "🟠 A space capsule showing significant deformations on its outer shell and cracked hatches.",
                "🟠 This aeronautical device shows atmospheric entry burn marks and scratched metals.",
                "🟠 A capsule deformed by a violent splashdown and water pressure, containing coarse sand."
            ],
            "worn": [
                "🟡 An Apollo space capsule whole although dirty, with its original conical structure well identifiable.",
                "🟡 This device has kept its suspension rings and metal attachments despite minor scratches.",
                "🟡 A used space capsule recovered then lost during transport, resting at the bottom of the water."
            ]
        },
        "fun_fact": "The Apollo space capsule is historical aerospace waste composed of aluminum and stainless steel honeycomb structures, insulated with epoxy resins and fiberglass, with a phenolic resin heat shield. If it ends up in water, the heat shield degrades slowly, releasing toxic phenolic compounds. The metallic structure resists very well, while the volume of the capsule strongly disrupts the water flow. Its lifting belongs to the heavy WEEE metal recycling at the landfill."
    },
    {
        "id": 240,
        "descriptions": {
            "badly damaged": [
                "🔴 A martian rover whose titanium chassis is broken in two, the metal wheels bent and blocked.",
                "🔴 This device shows broken optical cameras and eaten silicon chips covered with black mud.",
                "🔴 Plastic and printed circuit board fragments from a martian rover, lying in the bottom clayey silt."
            ],
            "damaged": [
                "🟠 A martian rover prototype showing significant cracks on its mechanical arm and shell.",
                "🟠 This technological device shows a broken solar panel and clogged steering gears.",
                "🟠 A rover deformed by a violent mechanical impact under water, containing sand and fine silt."
            ],
            "worn": [
                "🟡 A demonstration martian rover whole although very dirty, with its six wheels still in place.",
                "🟡 This device has kept its metal panoramic camera despite light signs of oxidation.",
                "🟡 A used martian rover discarded after secret tests in an aquatic environment, resting near the bank."
            ]
        },
        "fun_fact": "The martian rover (prototype) is exceptional technological waste. Made of titanium, composite carbon fiber, and equipped with high-performance rechargeable batteries and cutting-edge scientific equipment (spectrometers, lasers), its immersion is worrying. The electronic components contain toxic semiconductors and its battery can release harmful fluorinated solvents. This extraordinary waste must be extracted with the greatest care by specialists to join the landfill."
    },
    {
        "id": 241,
        "descriptions": {
            "badly damaged": [
                "🔴 An inert reactor core in lead and heavy steel completely split by corrosion and rusted.",
                "🔴 This massive container shows lead walls crumbling and an interior filled with toxic black mud.",
                "🔴 An inert reactor casing encrusted with hard marine sediments and filamentous algae."
            ],
            "damaged": [
                "🟠 A reactor core showing major cracks on its thick walls and bent safety valves.",
                "🟠 This container shows red rust plates and gray oxidation marks on its lead shielding.",
                "🟠 A metallic block deformed by extreme bottom forces, containing sand and foul silt."
            ],
            "worn": [
                "🟡 An inert lead and steel reactor core whole, although dirty and showing light signs of wear.",
                "🟡 This container has kept its heavy handling handles despite superficial scratches.",
                "🟡 A used inert reactor core discarded secretly, resting flat on the fine sand bed."
            ]
        },
        "fun_fact": "The inert reactor core consists of a very thick stainless steel casing, shielded with lead plates to protect against radiation. Although the nuclear fuel has been removed (making the core inert), the presence of tons of lead represents a colossal ecological risk. Lead is a highly toxic heavy metal that does not degrade and causes lead poisoning in fish, blocking their reproduction. Its military extraction is urgent to be processed at the landfill."
    },
    {
        "id": 242,
        "descriptions": {
            "badly damaged": [
                "🔴 A Pioneer probe plaque completely folded in two, the engraved surface scratched and unreadable.",
                "🔴 This aluminum panel shows torn edges and a surface encrusted with algae and black silt.",
                "🔴 Golden space plaque aluminum shards, half-buried under river bottom gravel."
            ],
            "damaged": [
                "🟠 A space plaque made of aluminum showing deformations on its reflective surface.",
                "🟠 This golden plaque shows sand rubbing scratches that partially erase the human engravings.",
                "🟠 A space probe plaque deformed by the current, containing fine silt on its edges."
            ],
            "worn": [
                "🟡 A golden Pioneer probe plaque whole although very dirty, the engraved diagrams still visible.",
                "🟡 This aluminum plaque has kept its original golden yellow luster despite light signs of wear.",
                "🟡 A replica or fragment of a space plaque lost recently, resting on a bed of light pebbles."
            ]
        },
        "fun_fact": "The Pioneer probe plaque is a historical metallic document made of anodized aluminum coated with a thin layer of gold. It carries a graphical message from humanity intended for potential extraterrestrial intelligences. If it ends up in fresh or salt water, the gold perfectly protects the underlying aluminum from any active chemical corrosion. It is a cultural artifact of invaluable symbolic value that must be recovered and valued in the landfill."
    }
]

apply_updates(updates_fr, updates_en)
print("Batch 10 updated successfully!")
