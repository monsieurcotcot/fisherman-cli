# -*- coding: utf-8 -*-
from update_junk_util import apply_updates

updates_fr = [
    {
        "id": 143,
        "descriptions": {
            "badly damaged": [
                "🔴 Une pâte de cellulose grise et spongieuse qui se délite au moindre contact dans le courant.",
                "🔴 Un résidu de gobelet en carton totalement aplati, recouvert de vase noire et de moisissures visqueuses.",
                "🔴 Des lambeaux cartonnés noirs et déchirés où le logo publicitaire rouge est presque entièrement effacé."
            ],
            "damaged": [
                "🟠 Un gobelet en carton ramolli et déformé par l'humidité, fendu sur toute sa longueur.",
                "🟠 Ce gobelet de soda montre une étanchéité perdue, avec son opercule en plastique déchiré et plein de vase.",
                "🟠 Un contenant cartonné dont le col est écrasé et souillé par des dépôts argileux."
            ],
            "worn": [
                "🟡 Un gobelet en carton encore entier mais détrempé, flottant verticalement près de la berge.",
                "🟡 Ce gobelet jetable de fast-food a conservé sa forme cylindrique malgré quelques pliures de surface.",
                "🟡 Un gobelet en carton usagé perdu récemment, dont l'étiquette extérieure est encore lisible."
            ]
        },
        "fun_fact": "Le gobelet en carton de marque semble écologique à première vue, mais il cache une réalité préoccupante. Pour le rendre étanche à l'eau et aux boissons chaudes, les industriels le tapissent d'un mince film intérieur en polyéthylène. Lorsqu'il finit dans un fleuve ou dans la mer, les fibres de papier se décomposent en quelques semaines, mais ce film plastique persiste pendant des décennies. En se désagrégeant sous l'action mécanique des vagues et du sel, il se fragmente en milliers de microparticules de plastique invisibles qui pénètrent directement dans l'organisme des poissons et des coquillages filtreurs, polluant gravement la chaîne alimentaire."
    },
    {
        "id": 144,
        "descriptions": {
            "badly damaged": [
                "🔴 Un morceau de film plastique jauni par les UV, devenu cassant et couvert de petits mollusques fixés.",
                "🔴 Des lambeaux d'acétate transparent effilochés en filaments emmêlés dans des brindilles pourries.",
                "🔴 Une pellicule plastique rigide et craquelée, à moitié enfouie sous le sable et le gravier du lit."
            ],
            "damaged": [
                "🟠 Une pochette plastique transparente plissée et étirée, contenant une eau trouble et sablonneuse.",
                "🟠 Ce protège-document montre de profondes rayures opaques et des déchirures au niveau de sa soudure.",
                "🟠 Une pochette en plastique souple déformée par le courant, couverte d'un fin dépôt calcaire."
            ],
            "worn": [
                "🟡 Une pochette transparente presque intacte, glissante et couverte d'un léger voile de limon.",
                "🟡 Ce protège-document léger flotte juste sous la surface de l'eau à la manière d'une méduse synthétique.",
                "🟡 Une pochette en plastique souple rejetée récemment, encore lisse malgré quelques pliures."
            ]
        },
        "fun_fact": "La pochette plastique transparente, couramment utilisée pour archiver ou protéger des documents administratifs, est fabriquée en polypropylène ou en PVC léger. Son séjour prolongé dans l'eau de mer ou de rivière l'expose aux rayons ultraviolets solaires qui dégradent ses polymères. Elle devient alors cassante et se fragmente en morceaux de plus en plus petits. Ces microplastiques de couleur translucide miment le plancton ou de petites proies marines, trompant la vigilance des poissons et des oiseaux qui les ingèrent."
    },
    {
        "id": 145,
        "descriptions": {
            "badly damaged": [
                "🔴 Une fine pelure de papier grisâtre, devenue gluante et se déchirant en fibres infimes sous l'eau.",
                "🔴 Une bouillie informe de pâte de cellulose noircie par l'oxydation thermique marine.",
                "🔴 Un débris de papier chimique tout racorni, amalgamé à des dépôts de vase fétide."
            ],
            "damaged": [
                "🟠 Un ticket de caisse froissé en boule serrée, retenant des débris d'algues et du sable noir.",
                "🟠 Ce reçu de magasin montre une surface délavee où l'encre thermique a totalement disparu sous l'eau.",
                "🟠 Un papier thermique fendu et fragile, dont les fibres se détachent lentement au gré du courant."
            ],
            "worn": [
                "🟡 Un ticket de caisse en papier lisse encore lisible, rejeté récemment sur le sable humide de la rive.",
                "🟡 Ce reçu de transaction commerciale est légèrement plissé et décoloré sur ses bordures extérieures.",
                "🟡 Un ticket de caisse usagé flottant à la surface de l'eau douce, encore entier et identifiable."
            ]
        },
        "fun_fact": "Le ticket de caisse thermique pose un problème écologique majeur en raison de son revêtement chimique. Contrairement au papier classique, le papier thermique contient des révélateurs de couleur comme le Bisphénol A (BPA) ou le Bisphénol S (BPS). Lorsque ces tickets sont jetés dans la nature et entrent en contact avec l'eau, ces perturbateurs endocriniens se dissolvent rapidement et contaminent le milieu aquatique. Ils altèrent le système reproducteur des amphibiens et des poissons, même à des concentrations infimes. Ils doivent être mis en décharge."
    },
    {
        "id": 146,
        "descriptions": {
            "badly damaged": [
                "🔴 Un film de polypropylène froissé à l'extrême, piégé dans une touffe d'algues filamenteuses en décomposition.",
                "🔴 Une fine pellicule de plastique souple encroûtée d'une pellicule de graisse sombre et de limon.",
                "🔴 Des fragments minuscules de plastique brillant flottant en surface comme des confettis microscopiques."
            ],
            "damaged": [
                "🟠 Un film protecteur transparent de paquet de cigarettes déchiré, rempli d'une fine vase sablonneuse.",
                "🟠 Ce plastique souple montre des élongations importantes dues à la déformation mécanique des vagues.",
                "🟠 Une pellicule plastique translucide fêlée sur ses pliures, retenant des débris organiques."
            ],
            "worn": [
                "🟡 Une pellicule plastique de protection presque intacte, flottant discrètement entre deux eaux.",
                "🟡 Ce plastique d'emballage conserve sa géométrie rectangulaire malgré des froissements légers.",
                "🟡 Un film plastique fin rejeté récemment sur le limon de la berge par le ressac du cours d'eau."
            ]
        },
        "fun_fact": "La pellicule de paquet de cigarettes est faite de polypropylène orienté, un polymère plastique très léger et ultra-résistant. En raison de sa faible densité, ce déchet flotte très facilement à la surface ou entre deux eaux, voyageant sur de grandes distances dans les fleuves jusqu'à l'océan. Sous l'action du ressac et du sel, il se fragmente en microparticules qui absorbent d'autres polluants hydrophobes présents dans l'eau. Il est indispensable de le trier dans la poubelle jaune pour qu'il soit collecté."
    },
    {
        "id": 147,
        "descriptions": {
            "badly damaged": [
                "🔴 Un petit débris d'aluminium et de plastique froncé, dont les couleurs sont délavées par le sel.",
                "🔴 Un opercule de papier d'aluminium froissé en boule compacte et incrusté dans la vase argileuse.",
                "🔴 Des micro-débris métalliques brillants et déchirés, coincés entre deux galets au fond de l'eau."
            ],
            "damaged": [
                "🟠 Un emballage de chewing-gum plié montrant de légères déchirures et des dépôts de sédiments fins.",
                "🟠 Ce petit sachet d'emballage présente des rayures abrasives qui ont effacé la marque commerciale.",
                "🟠 Un papier d'aluminium d'emballage plié sur lui-même, contenant du sable et des micro-algues."
            ],
            "worn": [
                "🟡 Un emballage de chewing-gum métallique argenté encore brillant, flottant à la dérive.",
                "🟡 Ce petit emballage souple a conservé ses motifs colorés visibles malgré un léger séjour sous l'eau.",
                "🟡 Un papier de chewing-gum rejeté récemment, à peine froissé et reposant sur une pierre humide."
            ]
        },
        "fun_fact": "L'emballage de chewing-gum est souvent composé d'un complexe multicouche associant du papier, de l'aluminium et un film plastique protecteur. Cette structure hybride rend son recyclage particulièrement difficile et sa décomposition extrêmement lente. Dans l'eau, le papier se dissout mais le film d'aluminium et de plastique persiste pendant plus de cent ans. Sa brillance métallique attire de nombreux oiseaux marins qui le confondent avec de petits poissons brillants et s'empoisonnent en l'avalant."
    },
    {
        "id": 148,
        "descriptions": {
            "badly damaged": [
                "🔴 Un bouchon en plastique rigide décoloré par le soleil, dont le pas de vis est rongé et couvert d'algues brunes.",
                "🔴 Une capsule en PEHD fendue en deux, présentant des traces manifestes de morsures de poissons.",
                "🔴 Un anneau de plastique et son capuchon encroûtés de calcaire et de sédiments sableux durs."
            ],
            "damaged": [
                "🟠 Une capsule en plastique montrant des déformations dues à l'écrasement sous des galets de fond.",
                "🟠 Ce bouchon de bouteille a perdu sa couleur d'origine et présente des rayures rugueuses profondes.",
                "🟠 Un capuchon en plastique dur à moitié rempli de vase argileuse grise et de résidus organiques."
            ],
            "worn": [
                "🟡 Une capsule en plastique de couleur vive flottant à la surface de l'eau comme une petite bouée.",
                "🟡 Ce bouchon à pas de vis a conservé son joint d'étanchéité interne malgré quelques traces d'usure.",
                "🟡 Une capsule de bouteille jetée récemment, encore propre et déposée sur un lit de galets."
            ]
        },
        "fun_fact": "La capsule de bouteille en plastique, généralement fabriquée en polyéthylène haute densité (PEHD) ou en polypropylène (PP), figure parmi les déchets les plus fréquemment retrouvés sur les plages du monde entier. Sa petite taille et sa flottabilité élevée lui permettent de dériver très loin. En mer, les oiseaux marins, notamment les albatros, ingèrent ces capsules colorées qu'ils confondent avec de la nourriture, ce qui provoque des occlusions intestinales mortelles. Elle doit être jetée dans la poubelle jaune."
    },
    {
        "id": 149,
        "descriptions": {
            "badly damaged": [
                "🔴 Des mailles de nylon noirci effilochées et nouées autour d'une branche morte, formant un piège pour la faune.",
                "🔴 Un cordage synthétique lourd en train de se décomposer en microfibres libres sous le ressac.",
                "🔴 Une tresse de filet synthétique raide et encroûtée de vase vaseuse et de débris de feuilles."
            ],
            "damaged": [
                "🟠 Un morceau de filet de tennis présentant des nœuds distendus et des mailles rompues en plusieurs points.",
                "🟠 Ce fragment de cordage en nylon montre une forte décoloration et retient de nombreux débris végétaux.",
                "🟠 Des fils synthétiques tressés et déformés par la traction des courants fluviaux."
            ],
            "worn": [
                "🟡 Un fragment de filet de tennis aux mailles encore bien formées, flottant lâchement dans l'eau.",
                "🟡 Ce morceau de filet synthétique noir a conservé sa texture rigide malgré quelques éraflures superficielles.",
                "🟡 Un débris de filet de sport usagé rejeté récemment, reposant près de la rive sablonneuse."
            ]
        },
        "fun_fact": "Le morceau de filet de tennis est constitué de fibres de nylon ou de polyéthylène tressées de haute résistance, conçues pour résister aux intempéries et aux chocs. Une fois dans le milieu aquatique, ce filet devient un déchet extrêmement dangereux par le phénomène de la 'pêche fantôme'. Ses mailles robustes piègent continuellement les poissons, les oiseaux et les petits mammifères aquatiques qui s'y empêtrent et finissent par mourir. Il doit impérativement être récupéré et jeté en décharge pour y être incinéré."
    },
    {
        "id": 150,
        "descriptions": {
            "badly damaged": [
                "🔴 Un couvercle métallique tordu et perforé par la rouille, dont le joint en caoutchouc intérieur s'effrite.",
                "🔴 Une rondelle de métal encroûtée d'oxydation rousse et de sédiments calcaires durs au fond de l'eau.",
                "🔴 Un disque de fer blanc perforé par la corrosion marine, ne laissant qu'un anneau tranchant."
            ],
            "damaged": [
                "🟠 Un opercule métallique présentant de larges taches de rouille brune sur ses décors sérigraphiés.",
                "🟠 Ce couvercle de bocal montre des déformations structurelles et des bords coupants oxydés.",
                "🟠 Un opercule en métal dont le revêtement protecteur s'écaille, libérant des résidus de peinture."
            ],
            "worn": [
                "🟡 Un opercule métallique de pot de confiture propre mais présentant des éraflures superficielles.",
                "🟡 Ce couvercle en métal a conservé ses pas de vis circulaires intacts malgré un court séjour dans l'eau.",
                "🟡 Un opercule en fer blanc rejeté récemment, brillant encore sous l'eau sur un lit de galets clairs."
            ]
        },
        "fun_fact": "L'opercule de pot de confiture est un couvercle métallique fabriqué en fer blanc (acier recouvert d'une fine couche d'étain) doté d'un joint en plastique (plastisol) à l'intérieur pour assurer l'étanchéité. Bien que l'acier soit recyclable à l'infini, son rejet dans l'eau entraîne une corrosion rapide qui libère des oxydes de fer dans le biotope. De plus, les bords coupants du métal rouillé représentent un danger direct de blessure pour la faune aquatique. Il doit être trié dans la poubelle jaune."
    },
    {
        "id": 151,
        "descriptions": {
            "badly damaged": [
                "🔴 Une coque en plastique thermoformé craquelée et jaunie, à moitié enfouie dans un limon fétide.",
                "🔴 Un support en carton bouilli et déchiré encore collé à des débris de pellicule plastique transparente.",
                "🔴 Des fragments de plastique rigide d'emballage devenus opaques et cassants sous l'effet du soleil."
            ],
            "damaged": [
                "🟠 Un emballage de brosse à dents plié et entrouvert, emprisonnant de la vase fine et des débris d'algues.",
                "🟠 Ce blister montre un plastique transparent fêlé et un carton arrière décoloré et ramolli par l'eau.",
                "🟠 Un emballage thermoformé déformé par l'écrasement mécanique, retenant du sable et des graviers."
            ],
            "worn": [
                "🟡 Un emballage de brosse à dents vide et entier, flottant à la surface de l'eau comme une bulle.",
                "🟡 Ce blister en plastique rigide a conservé sa forme thermoformée malgré de légères éraflures.",
                "🟡 Un emballage plastique et carton jeté récemment, dont les deux parties sont encore scellées."
            ]
        },
        "fun_fact": "L'emballage de brosse à dents est un blister bimatière composé d'une coque moulée en plastique transparent (souvent du PET) et d'un support en carton imprimé. Lorsque ce déchet se retrouve dans l'eau, le carton se dissout en quelques jours, libérant des encres potentiellement toxiques, tandis que la coque en plastique mettra plusieurs siècles à se dégrader. Ce plastique se fragmente en microparticules nocives. Il convient de séparer le carton (poubelle jaune) du plastique pour un tri optimal."
    },
    {
        "id": 152,
        "descriptions": {
            "badly damaged": [
                "🔴 Un réseau de fils de plastique vert complètement emmêlé et étiré, piégeant des graviers et du limon.",
                "🔴 Un sac à mailles synthétiques déchiqueté en lanières fines qui flottent comme des tentacules.",
                "🔴 Des débris de filet plastique durcis par le calcaire et les micro-organismes marins."
            ],
            "damaged": [
                "🟠 Un filet à provisions présentant des mailles déchirées en plusieurs endroits et étirées par le courant.",
                "🟠 Ce sac en filet plastique montre une décoloration prononcée due à l'exposition prolongée aux UV.",
                "🟠 Un filet synthétique emmêlé contenant des coquilles vides et des résidus de feuilles aquatiques."
            ],
            "worn": [
                "🟡 Un filet à provisions en plastique entier mais froissé, flottant doucement entre deux eaux.",
                "🟡 Ce sac en filet vert a conservé sa poignée et sa structure alvéolée malgré des salissures superficielles.",
                "🟡 Un filet plastique usagé jeté récemment, dérivant à plat à la surface de l'eau claire."
            ]
        },
        "fun_fact": "Le filet à provisions en plastique est un sac léger fabriqué à partir de fils de polyéthylène extrudés et soudés en forme de mailles extensibles. Ce déchet représente un risque élevé d'enchevêtrement pour les animaux aquatiques. Les tortues marines, les poissons et les oiseaux peuvent s'y emprisonner les nageoires, le bec ou le cou, ce qui limite leurs mouvements et les condamne à la famine ou à l'asphyxie. Il doit être jeté dans la poubelle jaune pour être recyclé en granulés."
    }
]

updates_en = [
    {
        "id": 143,
        "descriptions": {
            "badly damaged": [
                "🔴 A gray and spongy cellulose pulp that disintegrates at the slightest contact in the current.",
                "🔴 A completely flattened paper cup residue, covered with black silt and slimy mold.",
                "🔴 Torn and black cardboard shreds where the red advertising logo is almost entirely washed away."
            ],
            "damaged": [
                "🟠 A paper cup softened and deformed by moisture, split along its entire length.",
                "🟠 This soda cup shows a loss of watertightness, with its plastic lid torn and filled with silt.",
                "🟠 A cardboard container whose neck is crushed and stained by clay deposits."
            ],
            "worn": [
                "🟡 A paper cup still whole but waterlogged, floating vertically near the riverbank.",
                "🟡 This disposable fast-food cup has kept its cylindrical shape despite some surface creases.",
                "🟡 A discarded paper cup lost recently, its outer branding label still legible."
            ]
        },
        "fun_fact": "The branded paper cup seems eco-friendly at first glance, but it hides a concerning reality. To make it leak-proof for water and hot drinks, manufacturers line it with a thin inner layer of polyethylene plastic. When it ends up in a river or the sea, the paper fibers decompose in a few weeks, but this plastic film persists for decades. As it breaks down under the mechanical action of waves and salt, it fragments into thousands of invisible microplastic particles that directly enter the bodies of fish and shellfish, seriously polluting the food chain."
    },
    {
        "id": 144,
        "descriptions": {
            "badly damaged": [
                "🔴 A piece of plastic film yellowed by UV rays, which has become brittle and covered with small attached mollusks.",
                "🔴 Shreds of clear acetate frayed into tangled filaments caught in rotting twigs.",
                "🔴 A rigid and cracked plastic sheet, half-buried under the sand and gravel of the riverbed."
            ],
            "damaged": [
                "🟠 A clear plastic sleeve creased and stretched, containing cloudy and sandy water.",
                "🟠 This document protector shows deep opaque scratches and tears at its welded seam.",
                "🟠 A flexible plastic sleeve deformed by the current, covered with a thin lime deposit."
            ],
            "worn": [
                "🟡 A clear plastic sleeve almost intact, slippery and covered with a thin veil of silt.",
                "🟡 This lightweight document protector floats just below the surface of the water like a synthetic jellyfish.",
                "🟡 A flexible plastic sleeve discarded recently, still smooth despite some creases."
            ]
        },
        "fun_fact": "The clear plastic sleeve, commonly used to archive or protect administrative documents, is made of polypropylene or lightweight PVC. Its prolonged stay in sea or river water exposes it to solar ultraviolet rays that degrade its polymers. It then becomes brittle and fragments into smaller and smaller pieces. These translucent plastic microparticles mimic plankton or small marine prey, deceiving the vigilance of fish and birds that ingest them, causing physical blockages."
    },
    {
        "id": 145,
        "descriptions": {
            "badly damaged": [
                "🔴 A thin slip of grayish paper, which has become slimy and tears into tiny fibers underwater.",
                "🔴 A shapeless mush of cellulose pulp blackened by thermal marine oxidation.",
                "🔴 A shriveled piece of chemical paper, mixed with deposits of foul silt."
            ],
            "damaged": [
                "🟠 A cash receipt crumpled into a tight ball, holding algae debris and black sand.",
                "🟠 This store receipt shows a washed-out surface where the thermal ink has completely disappeared under water.",
                "🟠 A split and fragile thermal paper, whose fibers are slowly detaching in the current."
            ],
            "worn": [
                "🟡 A cash receipt made of smooth paper still legible, recently discarded on the wet sand of the riverbank.",
                "🟡 This commercial transaction receipt is slightly creased and faded on its outer edges.",
                "🟡 A used cash receipt floating on the surface of fresh water, still whole and identifiable."
            ]
        },
        "fun_fact": "The thermal cash receipt poses a major ecological problem due to its chemical coating. Unlike standard paper, thermal paper contains color developers such as Bisphenol A (BPA) or Bisphenol S (BPS). When these receipts are discarded in nature and come into contact with water, these endocrine disruptors dissolve quickly and contaminate the aquatic environment. They alter the reproductive system of amphibians and fish, even at tiny concentrations. They must be sent to the landfill."
    },
    {
        "id": 146,
        "descriptions": {
            "badly damaged": [
                "🔴 A polypropylene film crumpled to the extreme, trapped in a decaying clump of filamentous algae.",
                "🔴 A thin layer of flexible plastic encrusted with a film of dark grease and silt.",
                "🔴 Tiny fragments of shiny plastic floating on the surface like microscopic confetti."
            ],
            "damaged": [
                "🟠 A torn clear protective film from a cigarette pack, filled with fine sandy silt.",
                "🟠 This flexible plastic shows significant elongation due to the mechanical deformation of the waves.",
                "🟠 A translucent plastic film cracked on its folds, holding organic debris."
            ],
            "worn": [
                "🟡 A protective plastic film almost intact, floating discretely between two water layers.",
                "🟡 This packaging plastic retains its rectangular geometry despite light crumpling.",
                "🟡 A thin plastic film recently washed onto the silt of the bank by the river's backwash."
            ]
        },
        "fun_fact": "The cigarette pack plastic film is made of oriented polypropylene, an ultra-resistant and very light plastic polymer. Because of its low density, this waste floats very easily on the surface or between two water layers, traveling long distances in rivers to the ocean. Under the action of surf and salt, it fragments into microparticles that absorb other persistent organic pollutants present in the water. It is essential to sort it into the yellow recycling bin."
    },
    {
        "id": 147,
        "descriptions": {
            "badly damaged": [
                "🔴 A small, gathered piece of aluminum and plastic debris, its colors faded by salt.",
                "🔴 A foil wrapper crumpled into a compact ball and embedded in clayey silt.",
                "🔴 Torn and shiny metallic micro-debris, wedged between two pebbles at the bottom of the water."
            ],
            "damaged": [
                "🟠 A chewing gum wrapper folded showing slight tears and deposits of fine sediment.",
                "🟠 This small packaging bag shows abrasive scratches that have erased the commercial brand.",
                "🟠 A foil wrapper folded on itself, containing sand and micro-algae."
            ],
            "worn": [
                "🟡 A shiny silver metallic chewing gum wrapper still bright, floating adrift.",
                "🟡 This small flexible wrapper has kept its colorful patterns visible despite a short stay underwater.",
                "🟡 A chewing gum paper discarded recently, barely crumpled and resting on a wet stone."
            ]
        },
        "fun_fact": "The chewing gum wrapper is often composed of a multi-layer complex combining paper, aluminum, and a protective plastic film. This hybrid structure makes its recycling particularly difficult and its decomposition extremely slow. In water, the paper dissolves but the aluminum and plastic film persists for more than a hundred years. Its metallic luster attracts many seabirds that mistake it for small shiny fish and poison themselves by swallowing it."
    },
    {
        "id": 148,
        "descriptions": {
            "badly damaged": [
                "🔴 A rigid plastic cap faded by the sun, its thread eroded and covered with brown algae.",
                "🔴 An HDPE cap split in two, showing clear signs of fish bites.",
                "🔴 A plastic ring and its cap encrusted with lime and hard sandy sediment."
            ],
            "damaged": [
                "🟠 A plastic cap showing deformations due to crushing under pebbles at the bottom.",
                "🟠 This bottle cap has lost its original color and shows deep, rough scratches.",
                "🟠 A hard plastic cap half-filled with gray clayey silt and organic residues."
            ],
            "worn": [
                "🟡 A brightly colored plastic cap floating on the water surface like a small buoy.",
                "🟡 This screw cap has kept its internal seal despite some signs of wear.",
                "🟡 A bottle cap discarded recently, still clean and deposited on a bed of pebbles."
            ]
        },
        "fun_fact": "The plastic bottle cap, usually made of high-density polyethylene (HDPE) or polypropylene (PP), is among the waste items most frequently found on beaches worldwide. Its small size and high buoyancy allow it to drift very far. At sea, seabirds, especially albatrosses, ingest these colored caps, mistaking them for food, which causes fatal intestinal blockages. It must be disposed of in the yellow recycling bin for recovery."
    },
    {
        "id": 149,
        "descriptions": {
            "badly damaged": [
                "🔴 Frayed blackened nylon mesh knotted around a dead branch, forming a trap for wildlife.",
                "🔴 A heavy synthetic rope decomposing into loose microfibers under the surf.",
                "🔴 A stiff braid of synthetic netting encrusted with muddy silt and leaf debris."
            ],
            "damaged": [
                "🟠 A piece of tennis net showing loose knots and meshes broken in several places.",
                "🟠 This fragment of nylon rope shows severe discoloration and holds many plant debris.",
                "🟠 Synthetic threads twisted and deformed by the pull of river currents."
            ],
            "worn": [
                "🟡 A fragment of tennis net with its mesh still well formed, floating loosely in the water.",
                "🟡 This piece of black synthetic net has kept its rigid texture despite some surface scratches.",
                "🟡 A piece of used sports net discarded recently, resting near the sandy bank."
            ]
        },
        "fun_fact": "The piece of tennis net is made of high-strength braided nylon or polyethylene fibers, designed to withstand weather and impacts. Once in the aquatic environment, this net becomes extremely dangerous due to the phenomenon of 'ghost fishing.' Its robust mesh continuously traps fish, birds, and small aquatic mammals that get entangled in it and eventually die. It must be recovered and discarded in the landfill bin to be safely incinerated."
    },
    {
        "id": 150,
        "descriptions": {
            "badly damaged": [
                "🔴 A metal lid twisted and perforated by rust, its inner rubber seal crumbling away.",
                "🔴 A metal washer encrusted with red oxidation and hard calcareous sediment at the bottom of the water.",
                "🔴 A tinplate disc perforated by marine corrosion, leaving only a sharp ring."
            ],
            "damaged": [
                "🟠 A metal lid showing large brown rust spots on its screen-printed decorations.",
                "🟠 This jar lid shows structural deformations and sharp oxidized edges.",
                "🟠 A metal lid whose protective coating is peeling off, releasing paint residues."
            ],
            "worn": [
                "🟡 A metal jam jar lid clean but showing superficial scratches.",
                "🟡 This metal lid has kept its circular screw threads intact despite a short stay in the water.",
                "🟡 A tinplate lid discarded recently, still shining underwater on a bed of light pebbles."
            ]
        },
        "fun_fact": "The jam jar lid is a metal lid made of tinplate (steel coated with a thin layer of tin) equipped with a plastic seal (plastisol) inside to ensure a tight fit. Although steel is infinitely recyclable, its discharge into water leads to rapid corrosion that releases iron oxides into the ecosystem. In addition, the sharp edges of the rusted metal pose a direct danger of injury to aquatic wildlife. It must be sorted into the yellow recycling bin."
    },
    {
        "id": 151,
        "descriptions": {
            "badly damaged": [
                "🔴 A cracked and yellowed thermoformed plastic shell, half-buried in foul silt.",
                "🔴 A boiled, torn cardboard backing still stuck to fragments of clear plastic film.",
                "🔴 Rigid plastic packaging fragments that have become opaque and brittle under the sun."
            ],
            "damaged": [
                "🟠 A toothbrush packaging folded and half-open, trapping fine silt and algae debris.",
                "🟠 This blister pack shows cracked clear plastic and a backing card softened by water.",
                "🟠 A thermoformed package deformed by mechanical crushing, holding sand and gravel."
            ],
            "worn": [
                "🟡 A toothbrush packaging empty and whole, floating on the water surface like a bubble.",
                "🟡 This rigid plastic blister has kept its thermoformed shape despite minor scratches.",
                "🟡 A plastic and cardboard package discarded recently, with both parts still sealed together."
            ]
        },
        "fun_fact": "The toothbrush packaging is a dual-material blister pack consisting of a molded transparent plastic shell (often PET) and a printed cardboard backing. When this waste ends up in water, the cardboard dissolves in a few days, releasing potentially toxic inks, while the plastic shell will take several centuries to degrade. This plastic fragments into harmful microparticles. It is advisable to separate the cardboard from the plastic for optimal sorting."
    },
    {
        "id": 152,
        "descriptions": {
            "badly damaged": [
                "🔴 A completely tangled and stretched network of green plastic threads, trapping gravel and silt.",
                "🔴 A synthetic mesh bag shredded into thin strips that float like tentacles.",
                "🔴 Plastic net debris hardened by lime and marine micro-organisms."
            ],
            "damaged": [
                "🟠 A plastic grocery net showing torn meshes in several places and stretched by the current.",
                "🟠 This synthetic mesh bag shows pronounced discoloration due to prolonged exposure to UV.",
                "🟠 A tangled synthetic net containing empty shells and residues of aquatic leaves."
            ],
            "worn": [
                "🟡 A plastic grocery net whole but crumpled, floating gently between two water levels.",
                "🟡 This green mesh bag has kept its handle and honeycomb structure despite minor surface dirt.",
                "🟡 A used plastic net discarded recently, drifting flat on the surface of the clear water."
            ]
        },
        "fun_fact": "The plastic grocery net is a lightweight bag made of extruded and welded polyethylene threads in an expandable mesh pattern. This waste represents a high risk of entanglement for aquatic animals. Sea turtles, fish, and birds can get their flippers, beaks, or necks trapped in it, which limits their movements and condemns them to starvation or suffocation. It must be discarded in the yellow recycling bin to be processed."
    }
]

apply_updates(updates_fr, updates_en)
print("Batch 1 updated successfully!")
