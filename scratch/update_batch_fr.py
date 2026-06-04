# -*- coding: utf-8 -*-
import json

JUNK_FR_PATH = "data/junk_data.json"

batch_items_fr = [
    {
        "id": 218,
        "name": "Carcasse de caravane pliante",
        "rarity": "epic",
        "bin": "decharge",
        "recycling_notoriety_bonus": 100,
        "recycling_notoriety_malus": 200,
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
        "name": "Réservoir de carburant industriel",
        "rarity": "epic",
        "bin": "decharge",
        "recycling_notoriety_bonus": 100,
        "recycling_notoriety_malus": 200,
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
        "name": "Éolienne miniature brisée",
        "rarity": "epic",
        "bin": "decharge",
        "recycling_notoriety_bonus": 100,
        "recycling_notoriety_malus": 200,
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
        "name": "Pelle mécanique (bras)",
        "rarity": "epic",
        "bin": "decharge",
        "recycling_notoriety_bonus": 100,
        "recycling_notoriety_malus": 200,
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
        "name": "Ancre de marine à jas",
        "rarity": "epic",
        "bin": "decharge",
        "recycling_notoriety_bonus": 100,
        "recycling_notoriety_malus": 200,
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
    },
    {
        "id": 223,
        "name": "Treuil de chalutier",
        "rarity": "epic",
        "bin": "decharge",
        "recycling_notoriety_bonus": 100,
        "recycling_notoriety_malus": 200,
        "descriptions": {
            "badly damaged": [
                "🔴 Un treuil métallique de pêche complètement rouillé, dont le tambour de câble est grippé et cassé.",
                "🔴 Ce bloc de fer s'effrite sous l'action corrosive marine, le câble en acier tressé s'effilochant.",
                "🔴 Une carcasse de treuil de chalutier encroûtée de vase noire, d'algues et de coquilles."
            ],
            "damaged": [
                "🟠 Un treuil de pêche présentant des déformations sur son moteur hydraulique et des pignons bloqués.",
                "🟠 Cet appareil montre des fuites d'huile d'engrenage toxique sur ses axes rouillés.",
                "🟠 Un treuil métallique déformé par des chocs violents contre des roches, couvert de limon."
            ],
            "worn": [
                "🟡 Un treuil métallique de chalutier entier bien que très sale, avec son levier de commande encore mobile.",
                "🟡 Cet équipement de pêche a conservé son câble d'acier enroulé malgré des traces de rouille.",
                "🟡 Un treuil de bateau usagé jeté récemment, reposant sur le flanc au fond de l'eau."
            ]
        },
        "fun_fact": "Le treuil de chalutier sert à enrouler les câbles d'acier lourds (funes) qui tractent les filets de pêche. Sa structure en fonte d'acier massive contient des engrenages baignant dans de l'huile de transmission. Le principal danger vient du câble d'acier tressé restant : s'il s'effiloche sous l'action de l'érosion, il crée des brins d'acier coupants comme des rasoirs qui mutilent les poissons. Ce déchet lourd doit être déposé dans la benne à métaux de la décharge."
    },
    {
        "id": 224,
        "name": "Carcasse de jet-ski",
        "rarity": "epic",
        "bin": "decharge",
        "recycling_notoriety_bonus": 100,
        "recycling_notoriety_malus": 200,
        "descriptions": {
            "badly damaged": [
                "🔴 Une coque de jet-ski en résine complètement éventrée et brisée, le moteur interne rouillé et plein de boue.",
                "🔴 Ce jet-ski montre des pales de turbine cassées et des mousses intérieures qui se détachent.",
                "🔴 Une carcasse de véhicule marin encroûtée d'algues filamenteuses et de sédiments sableux."
            ],
            "damaged": [
                "🟠 Une coque en composite résine de jet-ski présentant des fissures majeures et des trous sur le flanc.",
                "🟠 Cet appareil montre des traces d'hydrocarbures légères et une selle en plastique arrachée.",
                "🟠 Un jet-ski déformé par un échouage violent, contenant de l'eau stagnante et du sable."
            ],
            "worn": [
                "🟡 Une coque de jet-ski en fibre de verre entière bien que sale, flottant à plat sur l'eau.",
                "🟡 Ce véhicule marin a conservé ses poignées en plastique malgré des éraflures superficielles.",
                "🟡 Un jet-ski usagé jeté récemment dans le canal, reposant au fond près de la berge."
            ]
        },
        "fun_fact": "La carcasse de jet-ski est constituée de plastique renforcé de fibres de verre (PRFV), de résines polyesters et d'un moteur à essence à deux ou quatre temps. La coque en plastique composite ne se décompose jamais sous l'eau et s'érode très lentement en libérant des fibres de verre microscopiques irritantes. Les restes de carburant et d'huile polluent les couches de surface. Ce déchet volumineux doit être dépollué dans une casse spécialisée ou en décharge."
    },
    {
        "id": 225,
        "name": "Hélice de cargo en bronze",
        "rarity": "epic",
        "bin": "decharge",
        "recycling_notoriety_bonus": 100,
        "recycling_notoriety_malus": 200,
        "descriptions": {
            "badly damaged": [
                "🔴 Une hélice de cargo en bronze complètement tordue et fissurée, couverte d'un dépôt calcaire épais.",
                "🔴 Ce bloc métallique lourd montre des pales ébréchées sous l'action corrosive marine.",
                "🔴 Une hélice de cargo encroûtée de sédiments marins durs et de grosses colonies de coquillages."
            ],
            "damaged": [
                "🟠 Une hélice en bronze présentant des déformations importantes et une oxydation verte de surface.",
                "🟠 Cette hélice montre des rayures de frottement abrasif et un moyeu de transmission tordu.",
                "🟠 Une pièce métallique massive déformée par des contraintes mécaniques extrêmes, couverte de vase."
            ],
            "worn": [
                "🟡 Une hélice de cargo en bronze entière, bien que mouillée et présentant un aspect terni.",
                "🟡 Cet équipement de navire a conservé ses grandes pales intactes malgré des éraflures.",
                "🟡 Une hélice de cargo usagée perdue récemment, reposant au fond de l'eau claire."
            ]
        },
        "fun_fact": "L'hélice de cargo en bronze est fabriquée dans un alliage de cuivre et d'étain très résistant à la corrosion saline. Cette pièce métallique géante peut peser plusieurs tonnes. Sa chute écrase définitivement le biotope de fond sous son poids. En s'oxydant très lentement, elle libère des oxydes de cuivre qui possèdent des propriétés algicides et bactéricides puissantes, détruisant la microfaune benthique environnante. Elle doit rejoindre la benne à métaux de la décharge."
    },
    {
        "id": 226,
        "name": "Cabine de tracteur agricole",
        "rarity": "epic",
        "bin": "decharge",
        "recycling_notoriety_bonus": 100,
        "recycling_notoriety_malus": 200,
        "descriptions": {
            "badly damaged": [
                "🔴 Une cabine métallique de tracteur complètement rouillée et écrasée, les vitres brisées en éclats coupants.",
                "🔴 Ce châssis en tôle rouillée montre des montants tordus et un habitacle rempli de boue noire.",
                "🔴 Une cabine de tracteur agricole encroûtée de vase marécageuse sombre, de calcaire et de coquilles."
            ],
            "damaged": [
                "🟠 Une cabine présentant des déformations importantes sur ses portes et un toit métallique fendu.",
                "🟠 Cet équipement agricole montre des vitres en verre trempé fissurées et des traces de moisissures.",
                "🟠 Une cabine de tracteur déformée par des chocs violents, contenant du sable grossier."
            ],
            "worn": [
                "🟡 Une cabine de tracteur agricole en acier entière bien que sale, avec sa structure métallique identifiable.",
                "🟡 Cet équipement a conservé ses rétroviseurs en plastique malgré des éraflures de surface.",
                "🟡 Une cabine de tracteur usagée jetée récemment, reposant sur le flanc au fond de l'eau."
            ]
        },
        "fun_fact": "La cabine de tracteur agricole est composée d'une structure en tôle d'acier peinte, de vitres en verre de sécurité trempé et de garnissages intérieurs en plastique. Les débris de tôle rouillée et de vitres cassées présentent des risques physiques de blessures pour les grands mammifères marins et d'eau douce. Ce déchet encombrant de grande taille doit rejoindre la benne à métaux ou la section tout-venant de la décharge pour être démantelée."
    },
    {
        "id": 227,
        "name": "Ailes d'avion léger",
        "rarity": "legendary",
        "bin": "decharge",
        "recycling_notoriety_bonus": 200,
        "recycling_notoriety_malus": 400,
        "descriptions": {
            "badly damaged": [
                "🔴 Des ailes d'avion en aluminium complètement tordues et déchirées, le train d'atterrissage tordu et rouillé.",
                "🔴 Ce reste d'aéronef montre une structure en aluminium froissée et encroûtée de sédiments et d'algues.",
                "🔴 Des débris métalliques d'ailes d'avion à moitié enfouis sous le gravier et le limon noir."
            ],
            "damaged": [
                "🟠 Des ailes d'avion léger présentant un fuselage fendu et des volets métalliques bloqués par la corrosion.",
                "🟠 Cet équipement aéronautique montre des traces d'oxydation blanche et des câbles arrachés.",
                "🟠 Des voilures d'avion déformées par le courant rapide, contenant du sable et des sédiments."
            ],
            "worn": [
                "🟡 Des ailes d'avion léger en aluminium encore entières, bien que sales et montrant des éraflures.",
                "🟡 Cette structure aérospatiale a conservé ses réservoirs d'aile scellés malgré des traces d'usure.",
                "🟡 Des ailes d'avion usagées jetées récemment, reposant à plat au fond de l'eau."
            ]
        },
        "fun_fact": "Les ailes d'avion léger sont composées d'alliages d'aluminium aéronautique de haute performance (comme le duralumin), de titane et de câbles d'acier. L'aluminium s'oxyde très lentement en formant une couche d'alumine étanche. La grande envergure des ailes constitue une barrière physique qui modifie les courants hydrographiques et perturbe la migration naturelle des poissons d'eau douce. Ce déchet légendaire doit être évacué par des services de relevage professionnels."
    },
    {
        "id": 228,
        "name": "Wagon de train de marchandises",
        "rarity": "legendary",
        "bin": "decharge",
        "recycling_notoriety_bonus": 200,
        "recycling_notoriety_malus": 400,
        "descriptions": {
            "badly damaged": [
                "🔴 Un wagon de marchandises complètement écrasé et rouillé, dont les essieux en acier lourd sont grippés.",
                "🔴 Cet énorme wagon montre des tôles extérieures tordues et rongées par la corrosion du sel marin.",
                "🔴 Une carcasse de wagon de train encroûtée de sédiments marins durs, d'algues et de coquilles."
            ],
            "damaged": [
                "🟠 Un wagon de train présentant des déformations importantes sur ses portes et un toit métallique enfoncé.",
                "🟠 Ce wagon montre des traces de moisissures intérieures sévères et un intérieur rempli de sable.",
                "🟠 Un wagon métallique déformé par des courants de fond violents, contenant du limon calcaire."
            ],
            "worn": [
                "🟡 Un wagon de train de marchandises entier bien que très sale, avec ses essieux d'acier encore solides.",
                "🟡 Ce véhicule ferroviaire a conservé ses tampons métalliques malgré des éraflures de surface.",
                "🟡 Un wagon usagé jeté récemment, reposant à plat au fond de l'eau claire."
            ]
        },
        "fun_fact": "Le wagon de train de marchandises est une structure métallique gigantesque faite d'acier ferroviaire très épais. Sa présence au fond d'un cours d'eau perturbe radicalement la dynamique sédimentaire locale, accumulant le limon et créant des zones de stagnation d'eau. La lente corrosion de ses composants métalliques et des restes de graisses lubrifiantes sur les bogies polluent le biotope. Son démantèlement exige des moyens de génie civil lourds en décharge."
    },
    {
        "id": 229,
        "name": "Locomotive de manœuvre",
        "rarity": "legendary",
        "bin": "decharge",
        "recycling_notoriety_bonus": 200,
        "recycling_notoriety_malus": 400,
        "descriptions": {
            "badly damaged": [
                "🔴 Une locomotive diesel complètement rouillée et brisée, le moteur interne éventré et plein de boue noire.",
                "🔴 Ce monstre de fer montre des roues d'acier bloquées par la corrosion et des cabines détruites.",
                "🔴 Une carcasse de locomotive encroûtée d'algues épaisses, de sédiments durs et de coquilles."
            ],
            "damaged": [
                "🟠 Une locomotive présentant des déformations sur sa structure externe et des vitres en verre brisées.",
                "🟠 Cet engin montre des fuites d'hydrocarbures lourds formant une nappe grasse noire sur l'eau.",
                "🟠 Une locomotive déformée par un accident violent, contenant du sable et du limon dans son foyer."
            ],
            "worn": [
                "🟡 Une locomotive de manœuvre en acier entier bien que sale, avec sa structure de fer identifiable.",
                "🟡 Cet engin ferroviaire a conservé son réservoir métallique externe malgré des traces de rouille.",
                "🟡 Une locomotive usagée jetée récemment dans le canal, reposant au fond près de la berge."
            ]
        },
        "fun_fact": "La locomotive de manœuvre diesel abrite un moteur thermique de grande cylindrée, des réservoirs de gazole de plusieurs milliers de litres et des transformateurs électriques. C'est un déchet légendaire extrêmement polluant. Les fuites d'hydrocarbures et d'huiles de lubrification étouffent instantanément la vie aquatique et s'infiltrent dans les sols. Sa dépollution exige un dégrutage complexe et un passage dans un centre de traitement spécialisé."
    },
    {
        "id": 230,
        "name": "Carcasse d'hélicoptère civil",
        "rarity": "legendary",
        "bin": "decharge",
        "recycling_notoriety_bonus": 200,
        "recycling_notoriety_malus": 400,
        "descriptions": {
            "badly damaged": [
                "🔴 Un hélicoptère dont le fuselage en aluminium s'est éventré, révélant la turbine rouillée et pleine de vase.",
                "🔴 Cet appareil montre des pales de rotor brisées et des mousses isolantes qui se détachent.",
                "🔴 Une carcasse d'aéronef encroûtée d'algues filamenteuses, de sédiments durs et de coquilles."
            ],
            "damaged": [
                "🟠 Un hélicoptère civil présentant des fissures majeures sur sa structure externe et des vitres brisées.",
                "🟠 Cet appareil montre des traces de fuites de kérosène toxique formant des reflets sur l'eau.",
                "🟠 Un hélicoptère déformé par un crash violent, contenant de l'eau stagnante et du sable fin."
            ],
            "worn": [
                "🟡 Une carcasse d'hélicoptère en aluminium entière bien que sale, flottant à plat sur l'eau.",
                "🟡 Cet appareil a conservé ses patins d'atterrissage métalliques malgré des éraflures de surface.",
                "🟡 Un hélicoptère usagé jeté récemment, reposant au fond de l'eau claire près de la rive."
            ]
        },
        "fun_fact": "La carcasse d'hélicoptère civil est un encombrant métallique composé d'alliages légers d'aluminium et de magnésium, de fibres de carbone composites et d'équipements électroniques. Les matériaux composites de ses pales ne se décomposent jamais, se fragmentant en microfibres toxiques. Les restes de carburant d'aviation (kérosène) contaminent chimiquement le milieu. Ce déchet de grande envergure doit être évacué par des équipes de dépollution."
    },
    {
        "id": 231,
        "name": "Générateur de centrale (rotor)",
        "rarity": "legendary",
        "bin": "decharge",
        "recycling_notoriety_bonus": 200,
        "recycling_notoriety_malus": 400,
        "descriptions": {
            "badly damaged": [
                "🔴 Un rotor de turbine électrique géant complètement rouillé, dont les bobinages de cuivre sont verdis.",
                "🔴 Cet axe d'acier massif s'effrite en plaques d'oxyde de fer épaisses sous l'effet du sel marin.",
                "🔴 Un générateur de centrale encroûté de sédiments marins calcaires durs et de coquilles."
            ],
            "damaged": [
                "🟠 Un générateur présentant des déformations importantes sur ses ailettes et un axe central tordu.",
                "🟠 Ce rotor montre des traces de corrosion verte sur ses isolants en plastique dur et ses fils.",
                "🟠 Une pièce de centrale électrique déformée par des forces internes, couverte de vase fine."
            ],
            "worn": [
                "🟡 Un rotor de générateur en acier entier, bien que mouillé et montrant des traces d'oxydation.",
                "🟡 Cet axe métallique massif a conservé sa géométrie d'origine malgré des éraflures de surface.",
                "🟡 Un générateur de centrale usagé jeté récemment, reposant au fond de l'eau claire."
            ]
        },
        "fun_fact": "Le générateur de centrale (rotor) est une pièce industrielle colossale pesant plusieurs dizaines de tonnes. Sa présence au fond détruit instantanément le relief sédimentaire et écrase les herbiers marins. Composé d'acier allié à haute résistance et de tonnes de cuivre pur pour les enroulements, sa valeur de recyclage est extrêmement élevée, mais son extraction requiert des navires de levage de forte capacité de la marine ou des industries."
    },
    {
        "id": 232,
        "name": "Radar maritime géant",
        "rarity": "legendary",
        "bin": "decharge",
        "recycling_notoriety_bonus": 200,
        "recycling_notoriety_malus": 400,
        "descriptions": {
            "badly damaged": [
                "🔴 Un dôme de radar en composite brisé en morceaux coupants, l'électroniques interne rouillée.",
                "🔴 Cet équipement montre une antenne métallique tordue et écrasée couverte de vase noire.",
                "🔴 Un radar maritime encroûté de sédiments marins calcaires durs et de coquilles de moules."
            ],
            "damaged": [
                "🟠 Un radar maritime présentant des fissures importantes sur sa structure externe et des circuits oxydés.",
                "🟠 Ce dôme montre des traces d'humidité intérieures et des connexions électriques vertes.",
                "🟠 Une antenne de radar déformée par la pression de l'eau, contenant du sable grossier."
            ],
            "worn": [
                "🟡 Un dôme de radar en composite blanc entier, bien que mouillé et présentant des éraflures.",
                "🟡 Cet appareil a conservé son antenne rotative en plastique malgré des traces d'usure de surface.",
                "🟡 Un radar maritime usagé jeté récemment, reposant au fond de l'eau claire."
            ]
        },
        "fun_fact": "Le radar maritime géant comprend un dôme de protection (radôme) en composite de résine polyester et de fibres de verre, ainsi qu'une antenne rotative métallique et des composants hyperfréquences. Le dôme composite résiste indéfiniment aux éléments marins sans se dégrader. Les composants électroniques internes recèlent des substances toxiques (arsenic, plomb, béryllium) qui polluent chimiquement l'eau s'ils sont brisés."
    }
]

def update_file(file_path, items):
    with open(file_path, "r", encoding="utf-8") as f:
        data = json.load(f)
        
    for item in items:
        rarity = item["rarity"]
        item_id = item["id"]
        
        for r in list(data.keys()):
            data[r] = [x for x in data[r] if x["id"] != item_id]
            
        clean_item = {
            "id": item_id,
            "name": item["name"],
            "size_min": 0.0,
            "size_mean": 0.0,
            "size_sigma": 0.0,
            "force_pristine": None,
            "force_state": None,
            "descriptions": item["descriptions"],
            "fun_fact": item["fun_fact"],
            "bin": item["bin"],
            "recycling_notoriety_bonus": item["recycling_notoriety_bonus"],
            "recycling_notoriety_malus": item["recycling_notoriety_malus"]
        }
        
        if rarity not in data:
            data[rarity] = []
        data[rarity].append(clean_item)
        
    for r in data.keys():
        data[r] = sorted(data[r], key=lambda x: x["id"])
        
    with open(file_path, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=4, ensure_ascii=False)
    print(f"Updated {len(items)} items in {file_path}")

if __name__ == "__main__":
    update_file(JUNK_FR_PATH, batch_items_fr)
    print("Batch 6 French successfully updated!")
