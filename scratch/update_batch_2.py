# -*- coding: utf-8 -*-
from update_junk_util import apply_updates

updates_fr = [
    {
        "id": 153,
        "descriptions": {
            "badly damaged": [
                "🔴 Une cupule en plastique blanc mince broyée en éclats tranchants et mêlée à de la vase putride.",
                "🔴 Des débris de plastique rigide provenant d'un pot fendu, incrustés d'une pellicule de limon argileux noir.",
                "🔴 Un résidu de pot de yaourt déchiqueté dont le fond est troué, enseveli sous les sédiments sablonneux."
            ],
            "damaged": [
                "🟠 Un gobelet en plastique déformé par la chaleur, dont la languette supérieure métallique a été arrachée.",
                "🟠 Ce pot de yaourt blanc montre de larges fissures latérales et des parois très ramollies.",
                "🟠 Un pot en plastique souple froissé sur les côtés, contenant des dépôts de sable et de vase fine."
            ],
            "worn": [
                "🟡 Un pot de yaourt en plastique blanc intact, flottant verticalement à la surface de l'eau.",
                "🟡 Ce contenant cylindrique a conservé sa forme conique malgré quelques éraflures superficielles.",
                "🟡 Un pot de yaourt usagé jeté récemment, propre mais sans son opercule."
            ]
        },
        "fun_fact": "Le pot de yaourt vide est un déchet en plastique très commun fabriqué en polystyrène (PS). Bien que léger, ce matériau possède une résistance chimique élevée qui empêche sa biodégradation naturelle. Immergé dans l'eau d'un ruisseau ou d'un fleuve, il subit l'usure mécanique des vagues et des courants, finissant par se briser en petits fragments tranchants. Ces éclats de plastique sont ingérés par les poissons et les oiseaux aquatiques, causant de graves lésions internes et des blocages de l'appareil digestif. Il doit être trié dans le bac de recyclage jaune."
    },
    {
        "id": 154,
        "descriptions": {
            "badly damaged": [
                "🔴 Un flacon en plastique opaque complètement écrasé et fendu, dont le bouchon a disparu.",
                "🔴 Une bouteille en polyéthylène décolorée par les UV, encroûtée d'algues séchées et de sable grossier.",
                "🔴 Des restes de bouteille plastique rigide cassée en plusieurs morceaux coupants piégés dans de la boue."
            ],
            "damaged": [
                "🟠 Une bouteille de shampoing présentant des éraflures majeures et des déformations sur sa coque plastique.",
                "🟠 Ce flacon plastique montre une étiquette en papier qui se décolle en lambeaux délavés.",
                "🟠 Une bouteille opaque déformée par les chocs contre des galets, remplie d'une eau savonneuse stagnante."
            ],
            "worn": [
                "🟡 Une bouteille de shampoing en plastique blanc opaque, flottant sur le flanc à la surface.",
                "🟡 Ce flacon a conservé sa forme allongée et son clapet de fermeture malgré quelques rayures légères.",
                "🟡 Une bouteille de cosmétique usagée jetée récemment, encore fermée et étanche."
            ]
        },
        "fun_fact": "La bouteille de shampoing vide est composée de polyéthylène haute densité (PEHD) ou de polypropylène (PP), des polymères conçus pour leur durabilité mécanique et chimique. Dans les fleuves, elle dérive en surface et libère lentement les résidus de shampoing qu'elle contient. Ces tensioactifs dégradent la qualité de l'eau en formant des mousses qui limitent l'oxygénation de la faune aquatique. Il faut impérativement la vider complètement et la trier dans la poubelle jaune pour permettre sa transformation en nouveaux objets plastiques."
    },
    {
        "id": 155,
        "descriptions": {
            "badly damaged": [
                "🔴 Un petit tube en aluminium tordu et percé, dont la peinture extérieure s'effrite en paillettes grises.",
                "🔴 Ce tube de colle plastique est écrasé et encroûté de sédiments argileux séchés.",
                "🔴 Des fragments de métal déchiquetés, soudés au limon de fond par de la colle polymérisée dure."
            ],
            "damaged": [
                "🟠 Un tube de colle en plastique montrant des pliures nettes et une buse obstruée par des résidus jaunis.",
                "🟠 Ce tube de colle montre de profondes rayures abrasives et un opercule métallique intérieur percé.",
                "🟠 Un tube souple écrasé en plusieurs points, présentant des coulures de colle séchée grises."
            ],
            "worn": [
                "🟡 Un tube de colle forte miniature en aluminium, vide mais avec son bouchon en plastique rouge.",
                "🟡 Ce tube de colle a conservé sa forme cylindrique d'origine malgré quelques pliures de surface.",
                "🟡 Un tube de colle usagé jeté récemment, flottant à la dérive près des berges vaseuses."
            ]
        },
        "fun_fact": "Le tube de colle vide constitue un déchet polluant en raison des résidus de solvants chimiques et d'adhésifs polymérisés qu'il contient encore. Qu'il soit en aluminium souple ou en plastique multicouche, il mettra plus d'un siècle à se détruire physiquement dans l'eau. Les solvants résiduels (esters, cyanoacrylates) peuvent se dissoudre lentement et empoisonner les micro-organismes aquatiques locaux. Ce déchet, en raison de son potentiel de toxicité chimique, doit être déposé dans la benne des déchets non recyclables en décharge."
    },
    {
        "id": 156,
        "descriptions": {
            "badly damaged": [
                "🔴 Un cordon de polyester effiloché en une multitude de fils lâches sales et incrustés de limon noir.",
                "🔴 Ce lacet synthétique s'est décomposé en morceaux de fibres éparpillés dans des racines immergées.",
                "🔴 Un lacet noirci et durci par les sédiments calcareux, enroulé étroitement autour d'une branche pourrie."
            ],
            "damaged": [
                "🟠 Un lacet de chaussure synthétique présentant des accrocs majeurs et des nœuds serrés impossibles à défaire.",
                "🟠 Ce cordon montre une décoloration prononcée due au soleil et des ferrets en plastique cassés.",
                "🟠 Un lacet en polyester encrassé de sable fin mouillé, ayant perdu toute sa souplesse d'origine."
            ],
            "worn": [
                "🟡 Un lacet de chaussure en polyester tressé rouge, encore solide bien que mouillé et sale.",
                "🟡 Ce lacet synthétique a conservé ses embouts plastiques intacts malgré quelques traces d'usure.",
                "🟡 Un lacet de chaussure usagé rejeté récemment, dérivant sous la surface de l'eau."
            ]
        },
        "fun_fact": "Le lacet de chaussure synthétique est composé de fibres de polyester ou de nylon tressées serrées. Ces polymères de synthèse sont insensibles à la décomposition biologique et résistent très bien à l'eau douce et salée. Au fil du temps, le frottement mécanique contre les sédiments fragmente le lacet en microfibres synthétiques microscopiques. Ces fibres persistent dans la colonne d'eau et sont ingérées par les mollusques filtreurs (comme les moules et les huîtres) qui s'intoxiquent. Ce lacet doit rejoindre la poubelle grise."
    },
    {
        "id": 157,
        "descriptions": {
            "badly damaged": [
                "🔴 Un bloc de polystyrène effrité en de petites billes blanches dispersées au gré du courant.",
                "🔴 Un morceau de mousse synthétique jauni, friable et colonisé par de la vase putride.",
                "🔴 Des débris de mousse polystyrène réduits à une texture spongieuse grise et cassante."
            ],
            "damaged": [
                "🟠 Un morceau de polystyrène cassé en plusieurs fragments, montrant des impacts de cailloux.",
                "🟠 Ce fragment de mousse blanche présente des trous abrasifs et des incrustations de sable noir.",
                "🟠 Un bloc de polystyrène expansé déformé par l'usure mécanique des vagues et couvert de vase."
            ],
            "worn": [
                "🟡 Un morceau de polystyrène expansé blanc et propre, flottant à la surface comme de l'écume.",
                "🟡 Ce fragment de mousse plastique a conservé sa rigidité et sa structure alvéolaire d'origine.",
                "🟡 Un bloc de polystyrène usagé flottant doucement, à peine jauni par la lumière du soleil."
            ]
        },
        "fun_fact": "Le morceau de polystyrène expansé est composé à 98% d'air piégé dans une structure de polymère de styrène. Très léger et friable, il se brise facilement sous l'action des courants et du vent en millions de petites billes blanches. Ces billes ressemblent à des œufs de poissons ou à des larves d'insectes, ce qui pousse les poissons à les manger. N'étant pas digestes, elles s'accumulent dans leur estomac, entraînant leur mort par inanition. Il doit être recyclé dans le bac jaune s'il est propre ou jeté en décharge."
    },
    {
        "id": 158,
        "descriptions": {
            "badly damaged": [
                "🔴 Un couvercle métallique de boîte de conserve très rouillé, plié en deux et couvert de limon noir.",
                "🔴 Ce couvercle en acier s'effrite en plaques d'oxyde de fer sous l'action corrosive marine.",
                "🔴 Un disque métallique d'emballage couvert de pustules d'oxydation rudes et de vase d'eau douce."
            ],
            "damaged": [
                "🟠 Un couvercle métallique de conserve présentant des arêtes tordues et très coupantes.",
                "🟠 Ce couvercle de boîte montre de larges taches de rouille rousse et une déformation de sa surface.",
                "🟠 Un couvercle métallique dont la languette d'ouverture facile s'est cassée ou détachée."
            ],
            "worn": [
                "🟡 Un couvercle de boîte de conserve en métal brillant, propre mais légèrement griffé.",
                "🟡 Ce couvercle métallique rond conserve ses rainures circulaires intactes malgré un séjour dans l'eau.",
                "🟡 Un opercule métallique de boîte de conserve jeté récemment, reposant sur un lit de galets."
            ]
        },
        "fun_fact": "Le couvercle de boîte de conserve est constitué d'acier recouvert d'une fine couche d'étain (fer blanc) ou d'aluminium. Abandonné dans un fleuve ou dans l'océan, il subit une corrosion chimique active. Bien que le fer et l'étain finissent par s'oxyder entièrement en formant des oxydes métalliques non toxiques, les bords extrêmement coupants du couvercle représentent un risque immédiat et mortel de mutilation pour les poissons et les plongeurs. Ce déchet métallique doit être jeté dans la poubelle jaune."
    },
    {
        "id": 159,
        "descriptions": {
            "badly damaged": [
                "🔴 Un blister de médicaments aplati et froissé, dont la feuille d'aluminium s'est déchirée en morceaux.",
                "🔴 Cet emballage alvéolé en plastique dur est devenu cassant et jauni, incrusté d'argile de rivière.",
                "🔴 Des restes d'emballage blister déchiquetés, flottant entre deux eaux parmi des algues vertes."
            ],
            "damaged": [
                "🟠 Un blister vide présentant plusieurs alvéoles plastiques enfoncées et percées.",
                "🟠 Cet emballage mixte montre une feuille d'aluminium partiellement pelée et couverte de limon.",
                "🟠 Un blister plastique transparent montrant des cassures sur ses bords rigides."
            ],
            "worn": [
                "🟡 Un blister de médicaments vide et propre, conservant la forme de ses alvéoles d'origine.",
                "🟡 Cet emballage plastique est froissé mais conserve sa feuille d'aluminium arrière brillante.",
                "🟡 Un blister de pilules vide jeté récemment, dérivant à la surface de l'eau."
            ]
        },
        "fun_fact": "Le blister de médicaments vide est un emballage composite constitué de plastique thermoformé (généralement du PVC) scellé à une mince pellicule d'aluminium. En raison de cette association intime de deux matériaux de nature différente, le blister est difficilement recyclable et nécessite un traitement spécifique. Sous l'eau, les deux couches finissent par se désolidariser lentement sous l'effet des vagues. L'aluminium s'érode doucement tandis que le PVC persiste pendant des siècles. Il doit être trié dans la poubelle jaune."
    },
    {
        "id": 160,
        "descriptions": {
            "badly damaged": [
                "🔴 Une paille en papier réduite à une fibre grise informe, se liquéfiant au moindre mouvement d'eau.",
                "🔴 Ce tube de papier en décomposition ressemble à une larve molle et décolorée sur le fond vaseux.",
                "🔴 Des lambeaux de fibres de papier collants et jaunis, agglomérés à du sable grossier."
            ],
            "damaged": [
                "🟠 Une paille en papier pliée et écrasée, dont les couches de carton se détachent en spirale.",
                "🟠 Cette paille montre des moisissures noires et s'est ramollie sous l'effet de l'eau.",
                "🟠 Un tube cartonné jetable gonflé d'eau, ayant perdu sa rigidité et sa couleur d'origine."
            ],
            "worn": [
                "🟡 Une paille en papier encore entière mais molle au toucher, flottant près de la berge.",
                "🟡 Ce chalumeau en papier coloré montre de légères rides d'humidité sur sa paroi cylindrique.",
                "🟡 Une paille en carton usagée jetée récemment, dérivant verticalement dans le courant."
            ]
        },
        "fun_fact": "La paille en papier détrempée a été introduite pour remplacer les pailles en plastique à usage unique interdites. Bien qu'elle soit fabriquée en cellulose et donc biodégradable en quelques mois dans l'eau, sa fabrication nécessite souvent des colles synthétiques et des encres colorées pour sa décoration. En se désagrégeant sous l'effet de l'eau, elle libère ces composés chimiques dans le milieu aquatique. De plus, elle met plusieurs semaines à disparaître et peut irriter les branchies des poissons. Elle va à la décharge."
    },
    {
        "id": 161,
        "descriptions": {
            "badly damaged": [
                "🔴 Un manche de brosse en plastique dur cassé en deux, dont tous les picots se sont détachés ou ont été usés.",
                "🔴 Cette brosse à cheveux montre un coussinet en caoutchouc entièrement pourri et plein de vase noire.",
                "🔴 Des fragments de plastique rigide et de picots en nylon mélangés à des sédiments marins calcaires."
            ],
            "damaged": [
                "🟠 Une brosse à cheveux présentant des picots en nylon tordus et un manche rayé par l'abrasion des galets.",
                "🟠 Cet objet montre un dos en plastique fendillé par le soleil et encroûté de micro-algues.",
                "🟠 Une brosse de coiffure déformée par la pression de l'eau, contenant des débris de feuilles mortes."
            ],
            "worn": [
                "🟡 Une brosse à cheveux entière mais sale, dont la poignée en plastique présente quelques éraflures.",
                "🟡 Cet accessoire a conservé la majorité de ses picots de nylon malgré un séjour dans l'eau.",
                "🟡 Une brosse à cheveux usagée rejetée récemment, reposant sur le flanc au fond du canal."
            ]
        },
        "fun_fact": "La brosse à cheveux cassée combine souvent différents types de plastiques : un manche rigide en ABS ou en polystyrène, un coussinet souple en caoutchouc synthétique et des picots en nylon. Cette composition complexe fait d'elle un déchet non recyclable dans le circuit classique. Dans l'eau, ses composants se séparent lentement sous l'effet de la dégradation mécanique. Les picots fins en nylon se dispersent et peuvent blesser la faune aquatique en cas d'ingestion. Elle doit être déposée en décharge."
    },
    {
        "id": 162,
        "descriptions": {
            "badly damaged": [
                "🔴 Un seau de plage en plastique fêlé de haut en bas, dont l'anse métallique a rouillé et s'est détachée.",
                "🔴 Cet accessoire de jeu montre des parois en plastique jauni devenues cassantes comme du verre.",
                "🔴 Des débris de plastique rouge de seau de plage écrasé, à moitié enfouis sous le gravier de fond."
            ],
            "damaged": [
                "🟠 Un seau de plage présentant des déformations importantes et une anse en plastique tordue.",
                "🟠 Ce jouet montre des décolorations prononcées dues aux UV et des rayures profondes remplies de vase.",
                "🟠 Un seau en plastique déformé par l'écrasement mécanique, rempli de sédiments sableux lourds."
            ],
            "worn": [
                "🟡 Un seau de plage en plastique rouge entier bien que sale, flottant à l'envers à la surface.",
                "🟡 Ce jouet d'enfant a conservé sa poignée en plastique souple malgré des éraflures superficielles.",
                "🟡 Un seau en plastique usagé jeté récemment, reposant au fond près d'une berge herbeuse."
            ]
        },
        "fun_fact": "Le jouet de plage (seau) est fabriqué en polyéthylène ou en polypropylène coloré de faible densité, ce qui lui confère une grande flottabilité. Égaré sur le sable, il est facilement emporté par les marées ou le vent vers les cours d'eau. Exposé aux rayonnements solaires, le plastique subit une photo-dégradation active qui altère ses propriétés mécaniques. Il devient friable et libère des additifs chimiques (comme les colorants métalliques). Il doit être récupéré et déposé aux encombrants en décharge."
    }
]

updates_en = [
    {
        "id": 153,
        "descriptions": {
            "badly damaged": [
                "🔴 A thin white plastic cup crushed into sharp shards and mixed with putrid silt.",
                "🔴 Rigid plastic fragments from a split pot, encrusted with a film of black clayey silt.",
                "🔴 A shredded yogurt pot residue with a hole in the bottom, buried under sandy sediment."
            ],
            "damaged": [
                "🟠 A plastic cup deformed by heat, its upper metal lid torn away.",
                "🟠 This white yogurt pot shows wide side cracks and very softened walls.",
                "🟠 A flexible plastic pot crumpled on the sides, containing deposits of sand and fine silt."
            ],
            "worn": [
                "🟡 A whole white plastic yogurt pot, floating vertically on the surface of the water.",
                "🟡 This cylindrical container has kept its conical shape despite some superficial scratches.",
                "🟡 A used yogurt pot discarded recently, clean but without its lid."
            ]
        },
        "fun_fact": "The empty yogurt pot is a very common plastic waste item made of polystyrene (PS). Although lightweight, this material has high chemical resistance that prevents natural biodegradation. Submerged in a stream or river, it undergoes mechanical wear from waves and currents, eventually breaking into small sharp fragments. These plastic shards are ingested by fish and aquatic birds, causing severe internal lesions and blockages of the digestive tract. It must be sorted in the yellow bin."
    },
    {
        "id": 154,
        "descriptions": {
            "badly damaged": [
                "🔴 An opaque plastic bottle completely crushed and split, its cap missing.",
                "🔴 A polyethylene bottle faded by UV rays, encrusted with dried algae and coarse sand.",
                "🔴 Remains of a rigid plastic bottle broken into several sharp pieces trapped in mud."
            ],
            "damaged": [
                "🟠 A shampoo bottle showing major scratches and deformations on its plastic shell.",
                "🟠 This plastic flask shows a paper label peeling off in faded shreds.",
                "🟠 An opaque bottle deformed by impacts against pebbles, filled with stagnant soapy water."
            ],
            "worn": [
                "🟡 An opaque white plastic shampoo bottle, floating on its side at the surface.",
                "🟡 This flask has kept its elongated shape and flip cap despite some light scratches.",
                "🟡 A used cosmetic bottle discarded recently, still closed and watertight."
            ]
        },
        "fun_fact": "The empty shampoo bottle is made of high-density polyethylene (HDPE) or polypropylene (PP), polymers designed for mechanical and chemical durability. In rivers, it drifts on the surface and slowly releases the shampoo residues it contains. These surfactants degrade water quality by forming foams that limit the oxygenation of aquatic life. It is imperative to empty it completely and sort it in the yellow bin to allow its transformation into new plastic objects."
    },
    {
        "id": 155,
        "descriptions": {
            "badly damaged": [
                "🔴 A small aluminum tube twisted and pierced, its outer paint peeling off in gray flakes.",
                "🔴 This plastic glue tube is crushed and encrusted with dried clayey sediments.",
                "🔴 Metal fragments shredded and welded to the bottom silt by hard polymerized glue."
            ],
            "damaged": [
                "🟠 A plastic glue tube showing sharp folds and a nozzle blocked by yellowed residues.",
                "🟠 This glue tube shows deep abrasive scratches and a pierced inner metal seal.",
                "🟠 A flexible tube crushed in several places, showing gray dried glue runs."
            ],
            "worn": [
                "🟡 A miniature strong glue aluminum tube, empty but with its red plastic cap.",
                "🟡 This glue tube has kept its original cylindrical shape despite some surface folds.",
                "🟡 A used glue tube discarded recently, floating adrift near the muddy banks."
            ]
        },
        "fun_fact": "The empty glue tube constitutes a polluting waste item due to the residues of chemical solvents and polymerized adhesives it still contains. Whether made of flexible aluminum or multi-layer plastic, it will take more than a century to physically destroy itself in water. Residual solvents (esters, cyanoacrylates) can slowly dissolve and poison local aquatic microorganisms. This waste, due to its potential chemical toxicity, must be deposited in the non-recyclable bin at the landfill."
    },
    {
        "id": 156,
        "descriptions": {
            "badly damaged": [
                "🔴 A polyester cord frayed into a multitude of loose, dirty threads encrusted with black silt.",
                "🔴 This synthetic shoelace has broken down into fiber pieces scattered in submerged roots.",
                "🔴 A shoelace blackened and hardened by calcareous sediments, tightly wrapped around a rotting branch."
            ],
            "damaged": [
                "🟠 A synthetic shoelace showing major snags and tight knots impossible to untie.",
                "🟠 This cord shows pronounced discoloration from the sun and broken plastic tips.",
                "🟠 A polyester shoelace clogged with wet fine sand, having lost all its original flexibility."
            ],
            "worn": [
                "🟡 A red braided polyester shoelace, still strong although wet and dirty.",
                "🟡 This synthetic shoelace has kept its plastic tips intact despite some signs of wear.",
                "🟡 A used shoelace discarded recently, drifting below the surface of the water."
            ]
        },
        "fun_fact": "The synthetic shoelace is composed of tightly braided polyester or nylon fibers. These synthetic polymers are insensitive to biological decomposition and resist fresh and salt water very well. Over time, mechanical friction against sediments fragments the shoelace into microscopic synthetic microfibers. These fibers persist in the water column and are ingested by filtering mollusks (like mussels and oysters) that poison themselves. It must go in the gray bin."
    },
    {
        "id": 157,
        "descriptions": {
            "badly damaged": [
                "🔴 A polystyrene block crumbled into small white beads scattered in the current.",
                "🔴 A piece of yellowed synthetic foam, brittle and colonized by putrid silt.",
                "🔴 Polystyrene foam debris reduced to a gray, brittle, spongy texture."
            ],
            "damaged": [
                "🟠 A piece of polystyrene broken into several fragments, showing stone impacts.",
                "🟠 This white foam fragment shows abrasive holes and black sand encrustations.",
                "🟠 An expanded polystyrene block deformed by mechanical wave wear and covered with silt."
            ],
            "worn": [
                "🟡 A clean white piece of expanded polystyrene, floating on the surface like foam.",
                "🟡 This plastic foam fragment has kept its original rigidity and honeycomb structure.",
                "🟡 A used polystyrene block floating gently, barely yellowed by sunlight."
            ]
        },
        "fun_fact": "The piece of expanded polystyrene is composed of 98% air trapped in a styrene polymer structure. Very light and brittle, it breaks easily under currents and wind into millions of small white beads. These beads resemble fish eggs or insect larvae, prompting fish to eat them. Since they are not digestible, they accumulate in their stomachs, leading to death by starvation. It must be recycled in the yellow bin if clean or thrown in the landfill."
    },
    {
        "id": 158,
        "descriptions": {
            "badly damaged": [
                "🔴 A highly rusted metal can lid, folded in two and covered with black silt.",
                "🔴 This steel lid crumbles into iron oxide flakes under corrosive marine action.",
                "🔴 A metallic packaging disc covered with rough oxidation pustules and freshwater silt."
            ],
            "damaged": [
                "🟠 A metal can lid showing twisted and very sharp edges.",
                "🟠 This can lid shows large red rust spots and deformation of its surface.",
                "🟠 A metal lid whose easy-open tab has broken off or detached."
            ],
            "worn": [
                "🟡 A shiny metal can lid, clean but slightly scratched.",
                "🟡 This round metal lid keeps its circular grooves intact despite a stay in water.",
                "🟡 A metal can lid discarded recently, resting on a bed of pebbles."
            ]
        },
        "fun_fact": "The can lid is made of steel coated with a thin layer of tin (tinplate) or aluminum. Abandoned in a river or the ocean, it undergoes active chemical corrosion. Although the iron and tin eventually oxidize completely, forming non-toxic metal oxides, the extremely sharp edges of the lid represent an immediate and fatal risk of injury to fish and divers. This metallic waste must be thrown in the yellow recycling bin."
    },
    {
        "id": 159,
        "descriptions": {
            "badly damaged": [
                "🔴 A flattened and creased medicine blister pack, its aluminum foil torn to pieces.",
                "🔴 This hard plastic blister pack is brittle and yellowed, encrusted with river clay.",
                "🔴 Shredded blister pack remains, floating between two waters among green algae."
            ],
            "damaged": [
                "🟠 An empty blister pack showing several depressed and pierced plastic cells.",
                "🟠 This mixed package shows an aluminum foil partially peeled and covered with silt.",
                "🟠 A transparent plastic blister pack showing breaks on its rigid edges."
            ],
            "worn": [
                "🟡 A clean, empty medicine blister pack, retaining the shape of its original cells.",
                "🟡 This plastic packaging is creased but retains its shiny back aluminum foil.",
                "🟡 An empty pill blister pack discarded recently, drifting on the surface of the water."
            ]
        },
        "fun_fact": "The empty medicine blister pack is a composite package consisting of thermoformed plastic (usually PVC) sealed to a thin aluminum film. Due to this intimate combination of two different materials, the blister pack is difficult to recycle and requires specific treatment. Under water, the two layers eventually separate slowly under waves. The aluminum erodes slowly while the PVC persists for centuries. It must be sorted in the yellow bin."
    },
    {
        "id": 160,
        "descriptions": {
            "badly damaged": [
                "🔴 A paper straw reduced to a shapeless gray fiber, liquefying at the slightest water movement.",
                "🔴 This decomposing paper tube resembles a soft, faded larva on the muddy bottom.",
                "🔴 Shreds of sticky, yellowed paper fibers, agglomerated with coarse sand."
            ],
            "damaged": [
                "🟠 A paper straw folded and crushed, its cardboard layers peeling off in a spiral.",
                "🟠 This straw shows black mold and has softened due to the effect of water.",
                "🟠 A disposable cardboard tube swollen with water, having lost its rigidity and original color."
            ],
            "worn": [
                "🟡 A paper straw still whole but soft to the touch, floating near the bank.",
                "🟡 This colored paper straw shows slight moisture wrinkles on its cylindrical wall.",
                "🟡 A used cardboard straw discarded recently, drifting vertically in the current."
            ]
        },
        "fun_fact": "The soggy paper straw was introduced to replace banned single-use plastic straws. Although made of cellulose and therefore biodegradable in a few months in water, its manufacture often requires synthetic glues and colored inks for its decoration. By disintegrating under the effect of water, it releases these chemical compounds into the aquatic environment. In addition, it takes several weeks to disappear and can irritate fish gills. It goes to the landfill."
    },
    {
        "id": 161,
        "descriptions": {
            "badly damaged": [
                "🔴 A hard plastic brush handle broken in two, all its pins detached or worn away.",
                "🔴 This hairbrush shows a completely rotted rubber pad filled with black silt.",
                "🔴 Rigid plastic fragments and nylon pins mixed with calcareous marine sediments."
            ],
            "damaged": [
                "🟠 A hairbrush showing bent nylon pins and a handle scratched by pebble abrasion.",
                "🟠 This object shows a plastic back cracked by the sun and encrusted with micro-algae.",
                "🟠 A hairbrush deformed by water pressure, containing dead leaf debris."
            ],
            "worn": [
                "🟡 A hairbrush whole but dirty, its plastic handle showing some scratches.",
                "🟡 This accessory has kept most of its nylon pins despite a stay in the water.",
                "🟡 A used hairbrush discarded recently, resting on its side at the bottom of the canal."
            ]
        },
        "fun_fact": "The broken hairbrush often combines different types of plastics: a rigid handle made of ABS or polystyrene, a flexible pad made of synthetic rubber, and nylon pins. This complex composition makes it a non-recyclable waste in the classic circuit. In water, its components separate slowly under mechanical degradation. The fine nylon pins disperse and can injure aquatic fauna if ingested. It must be deposited in the landfill."
    },
    {
        "id": 162,
        "descriptions": {
            "badly damaged": [
                "🔴 A beach bucket cracked from top to bottom, its metal handle rusted and detached.",
                "🔴 This toy shows yellowed plastic walls that have become brittle like glass.",
                "🔴 Red plastic debris from a crushed beach bucket, half-buried under bottom gravel."
            ],
            "damaged": [
                "🟠 A beach bucket showing significant deformations and a bent plastic handle.",
                "🟠 This toy shows pronounced UV discoloration and deep scratches filled with silt.",
                "🟠 A plastic bucket deformed by mechanical crushing, filled with heavy sandy sediments."
            ],
            "worn": [
                "🟡 A red plastic beach bucket whole but dirty, floating upside down at the surface.",
                "🟡 This child's toy has kept its flexible plastic handle despite minor surface scratches.",
                "🟡 A used plastic bucket discarded recently, resting at the bottom near a grassy bank."
            ]
        },
        "fun_fact": "The beach toy (bucket) is made of low-density colored polyethylene or polypropylene, giving it high buoyancy. Lost on the sand, it is easily carried by tides or wind to waterways. Exposed to solar radiation, the plastic undergoes active photodegradation that alters its mechanical properties. It becomes brittle and releases chemical additives (such as metal dyes). It must be recovered and deposited at the landfill."
    }
]

apply_updates(updates_fr, updates_en)
print("Batch 2 updated successfully!")
