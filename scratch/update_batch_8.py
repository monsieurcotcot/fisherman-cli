# -*- coding: utf-8 -*-
from update_junk_util import apply_updates

updates_fr = [
    {
        "id": 213,
        "descriptions": {
            "badly damaged": [
                "🔴 Un portail en fer forgé complètement rouillé et disloqué, dont les volutes décoratives se détachent.",
                "🔴 Cet élément de clôture montre des charnières brisées et des barreaux d'acier rongés par la corrosion marine.",
                "🔴 Une structure de grille métallique encroûtée de sédiments marins calcaires et de moules de fond."
            ],
            "damaged": [
                "🟠 Un portail en fer forgé présentant des déformations importantes et des barreaux tordus.",
                "🟠 Cette grille métallique montre de larges plaques de rouille brune et des gonds bloqués.",
                "🟠 Un portail déformé par l'écrasement mécanique sous l'eau, couvert de vase et d'algues."
            ],
            "worn": [
                "🟡 Un portail en fer forgé noir entier bien que sale, reposant à plat au fond de l'eau.",
                "🟡 Cet élément métallique a conservé ses motifs décoratifs malgré quelques traces de rouille de surface.",
                "🟡 Un portail de clôture usagé jeté récemment, reposant près de la berge de rivière."
            ]
        },
        "fun_fact": "Le portail en fer forgé tordu est un déchet métallique volumineux fabriqué en fer forgé ou en acier doux. Son abandon au fond d'un cours d'eau perturbe physiquement l'écosystème en formant une barrière artificielle rigide qui peut entraver la circulation des gros poissons et piéger les débris flottants. Sous l'eau, le fer subit une corrosion active qui le transformera en rouille en quelques siècles. Bien que les oxydes de fer soient peu toxiques, ses pointes rouillées et ses arêtes brisées représentent un danger de blessure. Il doit être trié en décharge."
    },
    {
        "id": 214,
        "descriptions": {
            "badly damaged": [
                "🔴 Un radiateur en fonte complètement brisé en plusieurs éléments lourds rongés par la corrosion.",
                "🔴 Cet appareil de chauffage montre des parois de fonte s'effritant sous l'action corrosive de l'eau.",
                "🔴 Une carcasse métallique massive à moitié ensevelie sous de la boue fétide et des sédiments durs."
            ],
            "damaged": [
                "🟠 Un radiateur en fonte présentant des fissures profondes et des ailettes métalliques tordues.",
                "🟠 Cet équipement montre des dépôts calcaires épais et des robinets de raccordement rouillés.",
                "🟠 Un radiateur déformé par le poids de sédiments rocheux sous l'eau, couvert d'algues."
            ],
            "worn": [
                "🟡 Un radiateur en fonte entier bien que sale, reposant droit sur le lit de la rivière.",
                "🟡 Cet élément de chauffage a conservé sa structure à colonnes malgré des éraflures superficielles.",
                "🟡 Un radiateur usagé jeté récemment dans le canal, encroûté d'un léger voile de limon."
            ]
        },
        "fun_fact": "Le radiateur en fonte est un déchet métallique extrêmement lourd fait de fonte de fer moulée. Son immersion écrase instantanément le biotope rocheux ou sableux sous-jacent, détruisant les micro-organismes benthiques. La fonte de fer se corrode lentement en milieu aqueux, libérant des oxydes ferriques qui colorent localement les sédiments. Bien que ces oxydes ne soient pas toxiques, sa présence physique modifie les courants hydrographiques locaux. Ce déchet lourd doit être extrait par des moyens mécaniques et valorisé en décharge."
    },
    {
        "id": 215,
        "descriptions": {
            "badly damaged": [
                "🔴 Une poutre métallique IPN complètement rouillée, dont les ailes se détachent en plaques d'oxyde de fer.",
                "🔴 Ce profilé d'acier de construction est à moitié enterré dans de la boue argileuse séchée.",
                "🔴 Une poutre métallique tordue et encroûtée de vase noire, d'algues et de coquilles maritimes."
            ],
            "damaged": [
                "🟠 Une poutre en acier présentant des torsions extrêmes et des traces d'oxydation rousse profondes.",
                "🟠 Ce profilé montre des rayures de frottement abrasif et des rebords métalliques amincis par la corrosion.",
                "🟠 Une poutre IPN déformée par des forces tectoniques de fond de rivière, couverte de sédiments."
            ],
            "worn": [
                "🟡 Une poutre en acier IPN entière, bien que mouillée et légèrement piquée de rouille extérieure.",
                "🟡 Ce profilé métallique de construction a conservé sa section en I malgré des éraflures de surface.",
                "🟡 Une poutre en acier usagée jetée récemment, reposant au fond de l'eau claire."
            ]
        },
        "fun_fact": "La poutre en acier (IPN) est un profilé structurel lourd utilisé dans le bâtiment. Faite d'acier de construction à haute résistance, elle pèse plusieurs dizaines de kilos par mètre. Son abandon dans le lit d'un cours d'eau modifie le relief de fond et crée des obstacles physiques artificiels qui perturbent la dynamique sédimentaire et l'écoulement de l'eau. Bien que sa corrosion lente en oxydes de fer soit sans toxicité majeure, sa manipulation nécessite un levage lourd pour un dépôt en décharge."
    },
    {
        "id": 216,
        "descriptions": {
            "badly damaged": [
                "🔴 Un moteur de voiture complètement rouillé et encroûté de limon noir huileux, le carter fendu.",
                "🔴 Ce moteur thermique montre des pistons bloqués et des fils électriques de cuivre vert-de-gris.",
                "🔴 Une carcasse métallique de moteur à moitié ensevelie sous les sédiments rocheux de fond."
            ],
            "damaged": [
                "🟠 Un moteur diesel présentant des fissures sur son bloc et des fuites d'huile noire persistantes.",
                "🟠 Ce moteur montre des vannes cassées et des poulies tordues par des chocs mécaniques violents.",
                "🟠 Un bloc-moteur déformé par l'écrasement, rempli d'une boue huileuse noire et de sable."
            ],
            "worn": [
                "🟡 Un moteur de voiture diesel entier bien que très sale, avec son démarreur encore attaché.",
                "🟡 Ce bloc métallique a conservé sa structure générale malgré de larges traces d'oxydation de surface.",
                "🟡 Un moteur thermique usagé jeté récemment dans le canal, reposant au fond de l'eau."
            ]
        },
        "fun_fact": "Le moteur de voiture diesel est un déchet industriel massif composé de fonte d'acier (bloc-moteur), d'aluminium (culasse), de cuivre (bobinages d'alternateur) et de résidus de fluides toxiques (huile moteur de lubrification, liquide de refroidissement au glycol). C'est un déchet hautement polluant : les fuites d'huiles de vidange usagées libèrent des métaux lourds (plomb, zinc) et des hydrocarbures toxiques qui étouffent instantanément la vie aquatique et contaminent la nappe phréatique. Il doit être traité en décharge."
    },
    {
        "id": 217,
        "descriptions": {
            "badly damaged": [
                "🔴 Un essieu de voiture en acier lourd complètement rouillé, dont les moyeux de roues sont grippés.",
                "🔴 Ce pont métallique montre des bras de suspension brisés et des restes de freins de disques oxydés.",
                "🔴 Une structure de pont de voiture encroûtée de sédiments marins calcaires durs et d'algues."
            ],
            "damaged": [
                "🟠 Un pont de voiture présentant des torsions importantes et des arbres de transmission tordus.",
                "🟠 Cet équipement mécanique montre des traces prononcées de rouille brune sur ses bras de suspension.",
                "🟠 Un essieu métallique déformé par des chocs violents contre des récifs, couvert de vase."
            ],
            "worn": [
                "🟡 Un pont de voiture entier bien que sale, avec sa structure métallique en acier encore identifiable.",
                "🟡 Cet essieu a conservé ses disques de freins en métal malgré de légères traces d'oxydation.",
                "🟡 Un essieu de voiture usagé jeté récemment dans le canal, reposant au fond près de la berge."
            ]
        },
        "fun_fact": "Le pont de voiture (essieu) est une pièce de transmission et de suspension lourde fabriquée en acier forgé de haute résistance mécanique. Son immersion accidentelle ou volontaire dans un cours d'eau constitue une source de pollution par des oxydes de fer et de graisses lubrifiantes résiduelles (issues des soufflets de cardan ou du différentiel). La corrosion de ses parties articulées libère également des particules métalliques de friction. Ce déchet lourd doit être retiré pour rejoindre la benne à métaux de la décharge."
    },
    {
        "id": 218,
        "descriptions": {
            "badly damaged": [
                "🔴 Une carcasse de caravane pliante complètement écrasée, dont les parois en bois ont pourri et la toile s'effrite.",
                "🔴 Ce reste de caravane de camping montre un châssis en acier rouillé couvert de sédiments et de limon.",
                "🔴 Des débris métalliques et plastiques d'une caravane détruite, à moitié enfouis au fond du fleuve."
            ],
            "damaged": [
                "🟠 Une caravane pliante présentant un toit affaissé et des panneaux d'aluminium déchirés.",
                "🟠 Cette carcasse montre des pneus éclatés et encroûtés de calcaire et un habitacle plein d'eau.",
                "🟠 Une structure de caravane déformée par le courant, contenant du sable et des dépôts de vase."
            ],
            "worn": [
                "🟡 Une caravane pliante entière bien que sale, avec sa structure en bois et aluminium encore identifiable.",
                "🟡 Ce véhicule de loisir a conservé ses roues métalliques malgré des éraflures superficielles sur sa coque.",
                "🟡 Une carcasse de caravane usagée jetée récemment, reposant sur le flanc près de la berge."
            ]
        },
        "fun_fact": "La carcasse de caravane pliante est un déchet volumineux constitué de matériaux composites complexes (bois traité, profilés d'aluminium, panneaux en polyester renforcé de fibre de verre et mousse isolante). Dans l'eau, les produits chimiques de préservation du bois (sels de cuivre, chrome, arsenic) s'infiltrent lentement, empoisonnant les poissons. Les mousses et résines se dégradent en microplastiques de façon persistante. Elle doit être extraite par des moyens mécaniques et déposée aux encombrants en décharge."
    },
    {
        "id": 219,
        "descriptions": {
            "badly damaged": [
                "🔴 Un réservoir de carburant en acier lourd complètement éventré et rouillé, laissant échapper une boue noire de fioul.",
                "🔴 Cet énorme réservoir montre des parois métalliques rongées par l'oxydation calcaire marine.",
                "🔴 Une carcasse de citerne industrielle tordue et encroûtée d'algues épaisses et de vase sombre."
            ],
            "damaged": [
                "🟠 Un réservoir métallique présentant de larges fissures sur ses soudures et des enfoncements profonds.",
                "🟠 Cette citerne montre des fuites de fioul lourd formant des reflets irisés gras sur l'eau.",
                "🟠 Un réservoir de carburant déformé par des pressions de fond, rempli de sédiments sableux lourds."
            ],
            "worn": [
                "🟡 Un réservoir de carburant industriel cylindrique complet, bien que sale et présentant des éraflures.",
                "🟡 Cette citerne a conservé ses vannes de fermeture métalliques malgré des traces de rouille légères.",
                "🟡 Un réservoir de fioul usagé jeté récemment, reposant au fond près d'un banc de sable."
            ]
        },
        "fun_fact": "Le réservoir de carburant industriel, ayant contenu du fioul ou du gazole, représente une source majeure d'hydrocarbures persistants s'il fuit dans l'eau. Ces hydrocarbures forment un film imperméable en surface (marée noire locale) qui bloque la diffusion de l'oxygène atmosphérique dans l'eau, entraînant l'asphyxie des poissons et de la flore aquatique. Les composés aromatiques s'accumulent dans les graisses des mollusques et des poissons, les rendant impropres à la consommation."
    },
    {
        "id": 220,
        "descriptions": {
            "badly damaged": [
                "🔴 Une éolienne dont les pales en fibre de carbone sont brisées, l'alternateur métallique interne rouillé.",
                "🔴 Cet appareil montre un mât métallique tordu et écrasé par la force des tempêtes d'eau.",
                "🔴 Des débris de génératrice éolienne encroûtés d'algues et de coquilles de moules."
            ],
            "damaged": [
                "🟠 Une éolienne de jardin présentant des pales cassées et des fissures importantes sur sa génératrice.",
                "🟠 Ce bloc montre des bobinages de cuivre vert oxydés et une dérive de direction arrachée.",
                "🟠 Une mini-éolienne déformée par un choc violent, contenant du limon sableux dans son pivot."
            ],
            "worn": [
                "🟡 Une éolienne miniature entière bien que sale, avec sa dérive en plastique encore en place.",
                "🟡 Cet appareil a conservé son alternateur étanche malgré des éraflures superficielles sur son mât.",
                "🟡 Une éolienne usagée jetée récemment dans le canal, reposant au fond près de la berge."
            ]
        },
        "fun_fact": "L'éolienne miniature brisée, bien que symbole de production d'énergie propre, devient un déchet polluant si elle est égarée dans un milieu sauvage. Ses pales sont constituées de résines époxy et de fibres de carbone ou de verre, des matériaux composites extrêmement résistants à la biodégradation. Son alternateur abrite des aimants permanents en terres rares (néodyme-dyprosium) et des bobinages de cuivre. Ce déchet technologique doit rejoindre la benne de tri des métaux de la décharge."
    },
    {
        "id": 221,
        "descriptions": {
            "badly damaged": [
                "🔴 Un bras de pelleteuse en acier lourd complètement tordu et rouillé, les flexibles hydrauliques arrachés.",
                "🔴 Cette pièce mécanique massive montre des articulations grippées par la corrosion et pleines de boue.",
                "🔴 Un bras d'excavatrice encroûté de sédiments marins durs et de colonies de coquillages."
            ],
            "damaged": [
                "🟠 Un bras de pelle mécanique présentant des fissures sur ses soudures d'acier et des bosses profondes.",
                "🟠 Ce bras métallique montre des fuites d'huile hydraulique toxique sur ses vérins rouillés.",
                "🟠 Une pièce de chantier déformée par des contraintes mécaniques extrêmes, couverte de vase."
            ],
            "worn": [
                "🟡 Un bras de pelle mécanique en acier lourd entier, bien que mouillé et montrant des traces d'oxydation.",
                "🟡 Cet équipement de chantier a conservé ses axes métalliques de fixation malgré des éraflures.",
                "🟡 Un bras de pelleteuse usagé jeté récemment, reposant au fond de l'eau claire."
            ]
        },
        "fun_fact": "Le bras de pelle mécanique est une pièce de structure massive en acier allié de haute résistance. Sa chute au fond de l'eau détruit mécaniquement le substrat rocheux ou sablonneux, écrasant les organismes benthiques et perturbant la morphologie du lit. Le principal risque de pollution provient de l'huile hydraulique résiduelle contenue dans ses vérins épais, qui libère des métaux et des additifs hydrocarbures toxiques. Il doit être recyclé en déchetterie par un ferrailleur."
    },
    {
        "id": 222,
        "descriptions": {
            "badly damaged": [
                "🔴 Une ancre marine en fer forgé complètement rouillée, dont le jas en bois a pourri et disparu.",
                "🔴 Ce lourd outil de mouillage s'effrite en plaques d'oxyde de fer épaisses sous l'effet du sel.",
                "🔴 Une ancre encroûtée de sédiments marins calcaires durs et de coquilles, soudée aux rochers de fond."
            ],
            "damaged": [
                "🟠 Une ancre de marine présentant des déformations sur ses pattes et une verge tordue.",
                "🟠 Cette ancre montre des plaques de rouille rousse et une manille de chaîne bloquée par la corrosion.",
                "🟠 Un équipement maritime lourd déformé par des chocs violents contre le récif, couvert de vase."
            ],
            "worn": [
                "🟡 Une ancre de marine à jas en fer noir entière, bien que mouillée et légèrement piquée de rouille.",
                "🟡 Cette ancre a conservé sa forme caractéristique malgré des éraflures de surface superficielles.",
                "🟡 Une ancre marine usagée perdue récemment, reposant à plat sur le lit de sable fin."
            ]
        },
        "fun_fact": "L'ancre de marine à jas est une pièce historique en fer forgé ou en acier coulé qui servait à ancrer les grands navires en bois. Dans l'eau de mer, le fer subit une corrosion galvanique lente qui libère des ions ferreux dans le milieu marin. Si ces ions ne sont pas toxiques à faible dose, la présence physique de l'ancre modifie localement les courants de fond et le substrat sableux. Ce déchet métallique lourd doit être retiré et valorisé en déchetterie (poubelle décharge)."
    }
]

updates_en = [
    {
        "id": 213,
        "descriptions": {
            "badly damaged": [
                "🔴 A wrought iron gate completely rusted and disjointed, its decorative scrolls peeling off.",
                "🔴 This fence element shows broken hinges and steel bars eaten away by marine corrosion.",
                "🔴 A metal grid structure encrusted with calcareous marine sediments and bottom mussels."
            ],
            "damaged": [
                "🟠 A wrought iron gate showing significant deformations and bent bars.",
                "🟠 This metal gate shows large plates of brown rust and seized hinges.",
                "🟠 A gate deformed by mechanical crushing under water, covered in silt and algae."
            ],
            "worn": [
                "🟡 A whole black wrought iron gate although dirty, resting flat at the bottom of the water.",
                "🟡 This metal element has kept its decorative patterns despite some surface rust marks.",
                "🟡 A used fence gate discarded recently, resting near the riverbank."
            ]
        },
        "fun_fact": "The bent wrought iron gate is bulky metallic waste made of wrought iron or mild steel. Its abandonment at the bottom of a waterway physically disrupts the ecosystem by forming a rigid artificial barrier that can impede the movement of large fish and trap floating debris. Under water, iron undergoes active corrosion that will transform it into rust in a few centuries. Although iron oxides are not very toxic, its rusted points and broken edges pose an injury hazard. It must go to the landfill."
    },
    {
        "id": 214,
        "descriptions": {
            "badly damaged": [
                "🔴 A cast iron radiator completely broken into several heavy elements eaten away by corrosion.",
                "🔴 This heating appliance shows cast iron walls crumbling under the corrosive action of water.",
                "🔴 A massive metal casing half-buried in foul mud and hard sediments."
            ],
            "damaged": [
                "🟠 A cast iron radiator showing deep cracks and bent metal fins.",
                "🟠 This equipment shows thick lime deposits and rusted connection valves.",
                "🟠 A radiator deformed by the weight of rocky sediments under water, covered in algae."
            ],
            "worn": [
                "🟡 A cast iron radiator whole although dirty, resting straight on the riverbed.",
                "🟡 This heating element has kept its column structure despite superficial scratches.",
                "🟡 A used radiator discarded recently in the canal, encrusted with a light veil of silt."
            ]
        },
        "fun_fact": "The cast iron radiator is extremely heavy metallic waste made of cast iron. Its immersion instantly crushes the underlying rocky or sandy biotope, destroying benthic microorganisms. Cast iron corrodes slowly in an aqueous medium, releasing ferric oxides that locally color the sediments. Although these oxides are not toxic, its physical presence alters local hydrographic currents. This heavy waste must be extracted and recovered in the scrap metal bin at the landfill."
    },
    {
        "id": 215,
        "descriptions": {
            "badly damaged": [
                "🔴 A steel IPN beam completely rusted, its flanges peeling off in iron oxide plates.",
                "🔴 This structural steel section is half-buried in dried clayey mud.",
                "🔴 A twisted steel beam encrusted with black silt, algae, and marine shells."
            ],
            "damaged": [
                "🟠 A steel beam showing extreme twists and deep red oxidation marks.",
                "🟠 This section shows abrasive rubbing scratches and metal edges thinned by corrosion.",
                "🟠 An IPN beam deformed by river bottom tectonic forces, covered in sediments."
            ],
            "worn": [
                "🟡 A steel IPN beam whole although wet and slightly pitted with external rust.",
                "🟡 This structural metal section has kept its I-shape despite surface scratches.",
                "🟡 A used steel beam discarded recently, resting at the bottom of the clear water."
            ]
        },
        "fun_fact": "The steel beam (IPN) is a heavy structural profile used in construction. Made of high-strength structural steel, it weighs several dozen kilos per meter. Its abandonment in a waterway bed alters the bottom relief and creates artificial physical obstacles that disrupt sediment dynamics and water flow. Although its slow corrosion into iron oxides is of no major toxicity, its handling requires heavy lifting for WEEE metal recycling at the landfill."
    },
    {
        "id": 216,
        "descriptions": {
            "badly damaged": [
                "🔴 A car engine completely rusted and encrusted with oily black silt, the crankcase split.",
                "🔴 This combustion engine shows blocked pistons and copper electrical wires green with verdigris.",
                "🔴 A metal engine casing half-buried under bottom rocky sediments."
            ],
            "damaged": [
                "🟠 A diesel engine showing cracks on its block and persistent black oil leaks.",
                "🟠 This engine shows broken valves and pulleys bent by violent mechanical impacts.",
                "🟠 An engine block deformed by crushing, filled with oily black mud and sand."
            ],
            "worn": [
                "🟡 A diesel car engine whole although very dirty, with its starter still attached.",
                "🟡 This metal block has kept its general structure despite wide surface oxidation marks.",
                "🟡 A used engine discarded recently in the canal, resting at the bottom of the water."
            ]
        },
        "fun_fact": "The diesel car engine is massive industrial waste composed of steel cast iron (engine block), aluminum (cylinder head), copper (alternator windings), and toxic fluid residues (lubricating engine oil, glycol coolant). It is highly polluting waste: leaks of used engine oil release heavy metals (lead, zinc) and toxic hydrocarbons that instantly suffocate aquatic life and contaminate groundwater. It must be treated and recycled at the landfill."
    },
    {
        "id": 217,
        "descriptions": {
            "badly damaged": [
                "🔴 A heavy steel car axle completely rusted, its wheel hubs seized.",
                "🔴 This metal axle shows broken suspension arms and residues of oxidized brake discs.",
                "🔴 A car axle structure encrusted with hard calcareous marine sediments and algae."
            ],
            "damaged": [
                "🟠 A car axle showing significant twists and bent drive shafts.",
                "🟠 This mechanical equipment shows pronounced brown rust marks on its suspension arms.",
                "🟠 A metal axle deformed by violent impacts against reefs, covered in silt."
            ],
            "worn": [
                "🟡 A car axle whole although dirty, with its steel structure still identifiable.",
                "🟡 This axle has kept its metal brake discs despite light signs of oxidation.",
                "🟡 A used car axle discarded recently in the canal, resting at the bottom near the bank."
            ]
        },
        "fun_fact": "The car axle (or differential bridge) is a heavy transmission and suspension component made of forged steel of high mechanical strength. Its accidental or voluntary immersion in a waterway constitutes a source of pollution by iron oxides and residual lubricating greases (from universal joint boots or the differential). Joint corrosion also releases metallic friction particles. This heavy waste must be removed to join the scrap metal bin at the landfill."
    },
    {
        "id": 218,
        "descriptions": {
            "badly damaged": [
                "🔴 A folding caravan frame completely crushed, its wooden walls rotted and canvas crumbling.",
                "🔴 This camping trailer remnant shows a rusted steel chassis covered with sediments and silt.",
                "🔴 Metal and plastic debris from a destroyed caravan, half-buried at the bottom of the river."
            ],
            "damaged": [
                "🟠 A folding caravan showing a sagging roof and torn aluminum panels.",
                "🟠 This frame shows blown tires encrusted with lime and a cabin full of water.",
                "🟠 A caravan structure deformed by the current, containing sand and silt deposits."
            ],
            "worn": [
                "🟡 A folding caravan frame whole although dirty, with its wood and aluminum structure still identifiable.",
                "🟡 This recreational vehicle has kept its metal wheels despite superficial scratches on its shell.",
                "🟡 A used caravan frame discarded recently, resting on its side near the bank."
            ]
        },
        "fun_fact": "The folding caravan frame is bulky waste consisting of complex composite materials (treated wood, aluminum sections, polyester panels reinforced with fiberglass, and insulation foam). In water, wood preservation chemicals (copper, chromium, arsenic salts) seep out slowly, poisoning fish. Foams and resins degrade into microplastics. It must be extracted by mechanical means and deposited in the non-recyclable bin at the landfill."
    },
    {
        "id": 219,
        "descriptions": {
            "badly damaged": [
                "🔴 A heavy steel fuel tank completely split and rusted, leaking black fuel mud.",
                "🔴 This huge tank shows metal walls eaten away by marine calcareous oxidation.",
                "🔴 A industrial tank casing bent and encrusted with thick algae and dark silt."
            ],
            "damaged": [
                "🟠 A metal tank showing wide cracks on its welds and deep dents.",
                "🟠 This tank shows heavy fuel oil leaks forming greasy iridescent reflections on the water.",
                "🟠 A fuel tank deformed by bottom pressures, filled with heavy sandy sediments."
            ],
            "worn": [
                "🟡 A cylindrical industrial fuel tank complete although dirty and showing scratches.",
                "🟡 This tank has kept its metal shut-off valves despite light rust marks.",
                "🟡 A used oil tank discarded recently, resting at the bottom near a sandbank."
            ]
        },
        "fun_fact": "The industrial fuel tank, having contained fuel oil or diesel, represents a major source of persistent hydrocarbons if it leaks in water. These hydrocarbons form an impermeable film on the surface (local oil spill) that blocks the diffusion of atmospheric oxygen into the water, leading to the suffocation of fish and aquatic flora. Aromatic compounds accumulate in the fats of mollusks and fish, making them unfit for consumption. It must go to the landfill."
    },
    {
        "id": 220,
        "descriptions": {
            "badly damaged": [
                "🔴 A wind turbine whose carbon fiber blades are broken, the internal metal alternator rusted.",
                "🔴 This device shows a metal mast bent and crushed by the force of water storms.",
                "🔴 Wind generator debris encrusted with algae and mussel shells."
            ],
            "damaged": [
                "🟠 A garden wind turbine showing broken blades and major cracks on its generator.",
                "🟠 This block shows oxidized green copper windings and a torn direction tail.",
                "🟠 A mini-turbine deformed by a violent impact, containing sandy silt in its pivot."
            ],
            "worn": [
                "🟡 A miniature wind turbine whole although dirty, with its plastic tail still in place.",
                "🟡 This device has kept its waterproof alternator despite superficial scratches on its mast.",
                "🟡 A used wind turbine discarded recently in the canal, resting at the bottom near the bank."
            ]
        },
        "fun_fact": "The broken miniature wind turbine, although a symbol of clean energy production, becomes polluting waste if lost in a wild environment. Its blades are made of epoxy resins and carbon or glass fibers, composite materials extremely resistant to biodegradation. Its alternator houses permanent magnets made of rare earths (neodymium-dysprosium) and copper windings. This technological waste must join the metal scrap bin at the landfill."
    },
    {
        "id": 221,
        "descriptions": {
            "badly damaged": [
                "🔴 A heavy steel excavator arm completely bent and rusted, hydraulic hoses torn off.",
                "🔴 This massive mechanical part shows joints seized by corrosion and full of mud.",
                "🔴 An excavator arm encrusted with hard marine sediments and shell colonies."
            ],
            "damaged": [
                "🟠 An excavator arm showing cracks on its steel welds and deep dents.",
                "🟠 This metal arm shows toxic hydraulic oil leaks on its rusted cylinders.",
                "🟠 A construction site part deformed by extreme mechanical stress, covered in silt."
            ],
            "worn": [
                "🟡 A heavy steel excavator arm whole although wet and showing oxidation marks.",
                "🟡 This construction equipment has kept its metal attachment pins despite scratches.",
                "🟡 A used excavator arm discarded recently, resting at the bottom of the clear water."
            ]
        },
        "fun_fact": "The excavator arm is a massive structural part made of high-strength alloy steel. Its fall to the bottom of the water mechanically destroys the rocky or sandy substrate, crushing benthic organisms and disrupting the morphology of the bed. The main risk of pollution comes from the residual hydraulic oil contained in its thick cylinders, which releases toxic metals and hydrocarbon additives. It must be recycled at the landfill by a scrap metal dealer."
    },
    {
        "id": 222,
        "descriptions": {
            "badly damaged": [
                "🔴 A wrought iron marine anchor completely rusted, its wooden stock rotted and gone.",
                "🔴 This heavy mooring tool crumbles into thick iron oxide plates under the effect of salt.",
                "🔴 An anchor encrusted with hard marine calcareous sediments and shells, welded to the bottom rocks."
            ],
            "damaged": [
                "🟠 A marine anchor showing deformations on its flukes and a bent shank.",
                "🟠 This anchor shows red rust plates and a shackle pin blocked by corrosion.",
                "🟠 Heavy maritime equipment deformed by violent impacts against the reef, covered in silt."
            ],
            "worn": [
                "🟡 A black iron marine anchor whole although wet and slightly pitted with rust.",
                "🟡 This anchor has kept its characteristic shape despite superficial surface scratches.",
                "🟡 A used marine anchor lost recently, resting flat on the fine sand bed."
            ]
        },
        "fun_fact": "The stocked marine anchor is a historical piece made of wrought iron or cast steel that was used to anchor large wooden ships. In seawater, iron undergoes slow galvanic corrosion that releases ferrous ions into the marine environment. While these ions are not toxic at low doses, the physical presence of the anchor locally modifies bottom currents and the sandy substrate. This heavy metal waste must be removed and valued in the metal scrap bin at the landfill."
    }
]

apply_updates(updates_fr, updates_en)
print("Batch 8 updated successfully!")
