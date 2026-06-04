# -*- coding: utf-8 -*-
from update_junk_util import apply_updates

updates_fr = [
    {
        "id": 223,
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
                "🟡 Une cabine de tracteur agricole en acier entière bien que sale, avec sa structure métallique originale.",
                "🟡 Cet équipement a conservé ses rétroviseurs en plastique malgré des éraflures de surface.",
                "🟡 Une cabine de tracteur usagée jetée récemment, reposant sur le flanc au fond de l'eau."
            ]
        },
        "fun_fact": "La cabine de tracteur agricole est composée d'une structure en tôle d'acier peinte, de vitres en verre de sécurité trempé et de garnissages intérieurs en plastique. Les débris de tôle rouillée et de vitres cassées présentent des risques physiques de blessures pour les grands mammifères marins et d'eau douce. Ce déchet encombrant de grande taille doit rejoindre la benne à métaux ou la section tout-venant de la décharge pour être démantelée."
    },
    {
        "id": 227,
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
                "🟡 Une locomotive de manœuvre en acier entier bien que sale, avec sa structure de fer originale.",
                "🟡 Cet engin ferroviaire a conservé son réservoir métallique externe malgré des traces de rouille.",
                "🟡 Une locomotive usagée jetée récemment dans le canal, reposant au fond près de la berge."
            ]
        },
        "fun_fact": "La locomotive de manœuvre diesel abrite un moteur thermique de grande cylindrée, des réservoirs de gazole de plusieurs milliers de litres et des transformateurs électriques. C'est un déchet légendaire extrêmement polluant. Les fuites d'hydrocarbures et d'huiles de lubrification étouffent instantanément la vie aquatique et s'infiltrent dans les sols. Sa dépollution exige un dégrutage complexe et un passage dans un centre de traitement spécialisé."
    },
    {
        "id": 230,
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

updates_en = [
    {
        "id": 223,
        "descriptions": {
            "badly damaged": [
                "🔴 A trawler metal winch completely rusted, its cable drum seized and broken.",
                "🔴 This iron block crumbles under corrosive marine action, the braided steel cable fraying.",
                "🔴 A trawler winch casing encrusted with black silt, algae, and shells."
            ],
            "damaged": [
                "🟠 A fishing winch showing deformations on its hydraulic motor and blocked gears.",
                "🟠 This device shows toxic gear oil leaks on its rusted shafts.",
                "🟠 A metal winch deformed by violent impacts against rocks, covered in silt."
            ],
            "worn": [
                "🟡 A trawler metal winch whole although very dirty, with its control lever still movable.",
                "🟡 This fishing equipment has kept its wound steel cable despite rust marks.",
                "🟡 A used boat winch discarded recently, resting on its side at the bottom of the water."
            ]
        },
        "fun_fact": "The trawler winch is used to wind the heavy steel cables (trawl warps) that pull the fishing nets. Its massive cast steel structure contains gears bathing in transmission oil. The main danger comes from the remaining braided steel cable: if it frays under the action of erosion, it creates steel strands sharp as razors that mutilate fish. This heavy waste must be deposited in the metal scrap bin at the landfill."
    },
    {
        "id": 224,
        "descriptions": {
            "badly damaged": [
                "🔴 A resin jet-ski hull completely split and broken, the internal motor rusted and muddy.",
                "🔴 This jet-ski shows broken turbine blades and interior foams detaching.",
                "🔴 A marine vehicle casing encrusted with filamentous algae and sandy sediments."
            ],
            "damaged": [
                "🟠 A resin composite jet-ski hull showing major cracks and holes on the side.",
                "🟠 This device shows light traces of hydrocarbons and a torn plastic seat.",
                "🟠 A jet-ski deformed by a violent grounding, containing stagnant water and sand."
            ],
            "worn": [
                "🟡 A fiberglass jet-ski hull whole although dirty, floating flat on the water.",
                "🟡 This marine vehicle has kept its plastic handles despite minor surface scratches.",
                "🟡 A used jet-ski discarded recently in the canal, resting at the bottom near the bank."
            ]
        },
        "fun_fact": "The jet-ski hull is made of fiberglass reinforced plastic (FRP), polyester resins, and a two- or four-stroke gasoline engine. The composite plastic hull never decomposes under water and erodes very slowly, releasing irritating microscopic glass fibers. Fuel and oil residues pollute surface layers. This bulky waste must be depolluted in a specialized shipyard or landfill."
    },
    {
        "id": 225,
        "descriptions": {
            "badly damaged": [
                "🔴 A cargo ship bronze propeller completely bent and cracked, covered in thick lime.",
                "🔴 This heavy metal block shows chipped blades under corrosive marine action.",
                "🔴 A cargo ship propeller encrusted with hard marine sediments and large shell colonies."
            ],
            "damaged": [
                "🟠 A bronze propeller showing significant deformations and green surface oxidation.",
                "🟠 This propeller shows abrasive friction scratches and a bent transmission hub.",
                "🟠 A massive metal part deformed by extreme mechanical stress, covered in silt."
            ],
            "worn": [
                "🟡 A bronze cargo ship propeller whole although wet and showing a tarnished appearance.",
                "🟡 This ship equipment has kept its large blades intact despite scratches.",
                "🟡 A used cargo ship propeller lost recently, resting at the bottom of the clear water."
            ]
        },
        "fun_fact": "The cargo ship bronze propeller is manufactured from a copper and tin alloy highly resistant to saline corrosion. This giant metal part can weigh several tons. Its fall permanently crushes the bottom biotope under its weight. By oxidizing very slowly, it releases copper oxides that possess powerful algicidal and bactericidal properties, destroying the surrounding benthic microfauna. It must join the scrap metal bin at the landfill."
    },
    {
        "id": 226,
        "descriptions": {
            "badly damaged": [
                "🔴 A metal tractor cab completely rusted and crushed, glass windows shattered into sharp shards.",
                "🔴 This steel sheet frame shows bent posts and a cabin filled with black mud.",
                "🔴 An agricultural tractor cab encrusted with dark swampy silt, lime, and shells."
            ],
            "damaged": [
                "🟠 A cab showing significant deformations on its doors and a split metal roof.",
                "🟠 This agricultural equipment shows cracked tempered glass windows and mold traces.",
                "🟠 A tractor cab deformed by violent impacts, containing coarse sand."
            ],
            "worn": [
                "🟡 A steel agricultural tractor cab whole although dirty, with its original metal frame.",
                "🟡 This equipment has kept its plastic rearview mirrors despite surface scratches.",
                "🟡 A used tractor cab discarded recently, resting on its side at the bottom of the water."
            ]
        },
        "fun_fact": "The agricultural tractor cab is composed of a painted steel sheet structure, tempered safety glass windows, and plastic interior linings. Rusted sheet metal debris and broken glass pose physical hazards of injury to large marine and freshwater mammals. This large, bulky waste must join the scrap metal bin or the bulk section of the landfill to be dismantled."
    },
    {
        "id": 227,
        "descriptions": {
            "badly damaged": [
                "🔴 Aluminum aircraft wings completely bent and torn, landing gear twisted and rusted.",
                "🔴 This aircraft remnant shows a crumpled aluminum structure encrusted with sediments and algae.",
                "🔴 Metallic light aircraft wing debris half-buried under gravel and black silt."
            ],
            "damaged": [
                "🟠 Light aircraft wings showing a split fuselage and control surfaces blocked by corrosion.",
                "🟠 This aeronautical equipment shows white oxidation marks and torn cables.",
                "🟠 Aircraft wings deformed by fast currents, containing sand and sediments."
            ],
            "worn": [
                "🟡 Light aircraft wings made of aluminum still whole although dirty and showing scratches.",
                "🟡 This aerospace structure has kept its sealed wing tanks despite signs of wear.",
                "🟡 Used aircraft wings discarded recently, resting flat at the bottom of the water."
            ]
        },
        "fun_fact": "The light aircraft wings are composed of high-performance aeronautical aluminum alloys (such as duralumin), titanium, and steel cables. Aluminum oxidizes very slowly, forming a waterproof alumina layer. The large wingspan of the wings constitutes a physical barrier that modifies hydrographic currents and disrupts the natural migration of freshwater fish. This legendary waste must be evacuated by professional recovery services."
    },
    {
        "id": 228,
        "descriptions": {
            "badly damaged": [
                "🔴 A freight wagon completely crushed and rusted, its heavy steel axles seized.",
                "🔴 This huge wagon shows outer plates bent and eaten away by marine salt corrosion.",
                "🔴 A train wagon casing encrusted with hard marine sediments, algae, and shells."
            ],
            "damaged": [
                "🟠 A train wagon showing significant deformations on its doors and a crushed metal roof.",
                "🟠 This wagon shows signs of severe internal mold and an interior filled with sand.",
                "🟠 A metal wagon deformed by violent bottom currents, containing calcareous silt."
            ],
            "worn": [
                "🟡 A freight train wagon whole although very dirty, with its steel axles still solid.",
                "🟡 This railway vehicle has kept its metal buffers despite surface scratches.",
                "🟡 A used wagon discarded recently, resting flat at the bottom of the clear water."
            ]
        },
        "fun_fact": "The freight train wagon is a gigantic metal structure made of very thick railway steel. Its presence at the bottom of a watercourse radically disrupts the local sediment dynamics, accumulating silt and creating water stagnation zones. The slow corrosion of its metal components and the remains of lubricating greases on the bogies pollute the biotope. Its dismantling requires heavy civil engineering equipment at the landfill."
    },
    {
        "id": 229,
        "descriptions": {
            "badly damaged": [
                "🔴 A diesel locomotive completely rusted and broken, the internal engine split and full of black mud.",
                "🔴 This iron monster shows steel wheels blocked by corrosion and destroyed cabs.",
                "🔴 A locomotive casing encrusted with thick algae, hard sediments, and shells."
            ],
            "damaged": [
                "🟠 A locomotive showing deformations on its external structure and broken glass windows.",
                "🟠 This engine shows heavy hydrocarbon leaks forming a black greasy slick on the water.",
                "🟠 A locomotive deformed by a violent accident, containing sand and silt in its firebox."
            ],
            "worn": [
                "🟡 A diesel shunting locomotive made of steel whole although dirty, with its original iron frame.",
                "🟡 This railway engine has kept its external metal tank despite rust marks.",
                "🟡 A used locomotive discarded recently in the canal, resting at the bottom near the bank."
            ]
        },
        "fun_fact": "The diesel shunting locomotive houses a large displacement combustion engine, fuel tanks of several thousand liters, and electrical transformers. It is extremely polluting legendary waste. Leaks of hydrocarbons and lubricating oils instantly suffocate aquatic life and seep into soils. Its depollution requires complex heavy lifting and a stay in a specialized treatment center."
    },
    {
        "id": 230,
        "descriptions": {
            "badly damaged": [
                "🔴 A helicopter whose aluminum fuselage is split open, revealing a rusted turbine full of silt.",
                "🔴 This device shows broken rotor blades and insulating foams detaching.",
                "🔴 An aircraft casing encrusted with filamentous algae, hard sediments, and shells."
            ],
            "damaged": [
                "🟠 A civil helicopter showing major cracks on its external structure and broken windows.",
                "🟠 This device shows signs of toxic kerosene fuel leaks forming reflections on the water.",
                "🟠 A helicopter deformed by a violent crash, containing stagnant water and fine sand."
            ],
            "worn": [
                "🟡 An aluminum helicopter fuselage whole although dirty, floating flat on the water.",
                "🟡 This device has kept its metal landing skids despite surface scratches.",
                "🟡 A used helicopter discarded recently, resting at the bottom of the clear water near the bank."
            ]
        },
        "fun_fact": "The civil helicopter fuselage is bulky metallic waste composed of lightweight aluminum and magnesium alloys, composite carbon fibers, and electronic equipment. The composite materials of its blades never decompose, fragmenting into toxic microfibers. Residual aviation fuel (kerosene) chemically contaminates the environment. This large-scale waste must be evacuated by specialized depollution teams."
    },
    {
        "id": 231,
        "descriptions": {
            "badly damaged": [
                "🔴 A giant electrical turbine rotor completely rusted, its copper windings green.",
                "🔴 This massive steel shaft crumbles into thick iron oxide plates under sea salt action.",
                "🔴 A power plant generator encrusted with hard marine calcareous sediments and shells."
            ],
            "damaged": [
                "🟠 A generator showing significant deformations on its blades and a bent central shaft.",
                "🟠 This rotor shows green corrosion marks on its hard plastic insulators and wires.",
                "🟠 A power plant component deformed by internal forces, covered in fine silt."
            ],
            "worn": [
                "🟡 A steel generator rotor whole although wet and showing signs of oxidation.",
                "🟡 This massive metal shaft has kept its original geometry despite surface scratches.",
                "🟡 A used power plant generator discarded recently, resting at the bottom of the clear water."
            ]
        },
        "fun_fact": "The power plant generator (rotor) is a colossal industrial component weighing several dozen tons. Its presence at the bottom instantly destroys the sediment relief and crushes seagrass beds. Composed of high-strength alloy steel and tons of pure copper for the windings, its recycling value is extremely high, but its extraction requires heavy lifting vessels from the navy or marine industries."
    },
    {
        "id": 232,
        "descriptions": {
            "badly damaged": [
                "🔴 A composite radar dome broken into sharp pieces, the internal electronics rusted.",
                "🔴 This equipment shows a metal antenna bent and crushed covered in black silt.",
                "🔴 A marine radar encrusted with hard calcareous marine sediments and mussel shells."
            ],
            "damaged": [
                "🟠 A marine radar showing major cracks on its external structure and oxidized circuits.",
                "🟠 This dome shows internal moisture traces and green electrical connections.",
                "🟠 A radar antenna deformed by water pressure, containing coarse sand."
            ],
            "worn": [
                "🟡 A white composite radar dome whole although wet and showing scratches.",
                "🟡 This device has kept its rotating plastic antenna despite surface wear marks.",
                "🟡 A used marine radar discarded recently, resting at the bottom of the clear water."
            ]
        },
        "fun_fact": "The giant marine radar includes a protective dome (radome) made of polyester resin and fiberglass composite, as well as a rotating metal antenna and microwave components. The composite dome resists marine elements indefinitely without degrading. The internal electronic components house toxic substances (arsenic, lead, beryllium) that chemically pollute the water if broken."
    }
]

apply_updates(updates_fr, updates_en)
print("Batch 9 updated successfully!")
