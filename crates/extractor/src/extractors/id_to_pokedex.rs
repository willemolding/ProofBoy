//! A mapping from the pokemon red/blue specific ID to the globally known pokedex number and name

pub(crate) const ID_TO_POKEDEX: [(u8, &str); 191] = [
(000,"No Pokemon"),
(112,"Rhydon"),
(115,"Kangaskhan"),
(032,"Nidoran♂"),
(035,"Clefairy"),
(021,"Spearow"),
(100,"Voltorb"),
(034,"Nidoking"),
(080,"Slowbro"),
(002,"Ivysaur"),
(103,"Exeggutor"),
(108,"Lickitung"),
(102,"Exeggcute"),
(088,"Grimer"),
(094,"Gengar"),
(029,"Nidoran♀"),
(031,"Nidoqueen"),
(104,"Cubone"),
(111,"Rhyhorn"),
(131,"Lapras"),
(059,"Arcanine"),
(151,"Mew"),
(130,"Gyarados"),
(090,"Shellder"),
(072,"Tentacool"),
(092,"Gastly"),
(123,"Scyther"),
(120,"Staryu"),
(009,"Blastoise"),
(127,"Pinsir"),
(114,"Tangela"),
(000,"MissingNo. (Gyaoon)"),
(000,"MissingNo. (Nidoran♂-like Pokémon)"),
(058,"Growlithe"),
(095,"Onix"),
(022,"Fearow"),
(016,"Pidgey"),
(079,"Slowpoke"),
(064,"Kadabra"),
(075,"Graveler"),
(113,"Chansey"),
(067,"Machoke"),
(122,"Mr. Mime"),
(106,"Hitmonlee"),
(107,"Hitmonchan"),
(024,"Arbok"),
(047,"Parasect"),
(054,"Psyduck "),
(096,"Drowzee"),
(076,"Golem"),
(000,"MissingNo.(Balloonda)"),
(126,"Magmar"),
(000,"MissingNo.(Buu)"),
(125,"Electabuzz"),
(082,"Magneton"),
(109,"Koffing"),
(000,"MissingNo.(Deer)"),
(056,"Mankey"),
(086,"Seel"),
(050,"Diglett"),
(128,"Tauros"),
(000,"MissingNo.(Elephant Pokémon)"),
(000,"MissingNo.(Crocky)"),
(000,"MissingNo.(Squid Pokémon 1)"),
(083,"Farfetch'd"),
(048,"Venonat"),
(149,"Dragonite"),
(000,"MissingNo.(Cactus)"),
(000,"MissingNo.(Jaggu)"),
(000,"MissingNo.(Zubat pre-evo)"),
(084,"Doduo"),
(060,"Poliwag"),
(124,"Jynx"),
(146,"Moltres"),
(144,"Articuno"),
(145,"Zapdos"),
(132,"Ditto"),
(052,"Meowth"),
(098,"Krabby"),
(000,"MissingNo.(Fish Pokémon 1)"),
(000,"MissingNo.(Fish Pokémon 2)"),
(000,"MissingNo.(Vulpix pre-evo)"),
(037,"Vulpix"),
(038,"Ninetales"),
(025,"Pikachu"),
(026,"Raichu"),
(000,"MissingNo.(Frog-like Pokémon 1)"),
(000,"MissingNo.(Frog-like Pokémon 2)"),
(147,"Dratini"),
(148,"Dragonair"),
(140,"Kabuto"),
(141,"Kabutops"),
(116,"Horsea"),
(117,"Seadra"),
(000,"MissingNo.(Lizard Pokémon 2)"),
(000,"MissingNo.(Lizard Pokémon 3)"),
(027,"Sandshrew"),
(028,"Sandslash"),
(138,"Omanyte"),
(139,"Omastar"),
(039,"Jigglypuff"),
(040,"Wigglytuff"),
(133,"Eevee"),
(136,"Flareon"),
(135,"Jolteon"),
(134,"Vaporeon"),
(066,"Machop"),
(041,"Zubat"),
(023,"Ekans"),
(046,"Paras"),
(061,"Poliwhirl"),
(062,"Poliwrath"),
(013,"Weedle"),
(014,"Kakuna"),
(015,"Beedrill"),
(000,"MissingNo.[Unknown]"),
(085,"Dodrio"),
(057,"Primeape"),
(051,"Dugtrio"),
(049,"Venomoth"),
(087,"Dewgong"),
(000,"MissingNo.[Unknown]"),
(000,"MissingNo.(Squid Pokémon 2)"),
(010,"Caterpi"),
(011,"Metapod"),
(012,"Butterfree"),
(068,"Machamp"),
(000,"MissingNo.(Golduck mid-evo)"),
(055,"Golduck"),
(097,"Hypno"),
(042,"Golbat"),
(150,"Mewtwo"),
(143,"Snorlax"),
(129,"Magikarp"),
(000,"MissingNo.(Meowth pre-evo)"),
(000,"MissingNo.[Unknown]"),
(089,"Muk"),
(000,"MissingNo.(Gyaoon pre-evo)"),
(099,"Kingler"),
(091,"Cloyster"),
(000,"MissingNo.(Magneton-like Pokémon)"),
(101,"Electrode"),
(036,"Clefable"),
(110,"Weezing"),
(053,"Persian"),
(105,"Marowak"),
(000,"MissingNo.(Marowak evo)"),
(093,"Haunter"),
(063,"Abra"),
(065,"Alakazam"),
(017,"Pidgeotto"),
(018,"Pidgeot"),
(121,"Starmie"),
(001,"Bulbasaur"),
(003,"Venusaur"),
(073,"Tentacruel"),
(000,"MissingNo.Goldeen pre-evo)"),
(118,"Goldeen"),
(119,"Seaking"),
(000,"MissingNo.(Kotora)"),
(000,"MissingNo.(Raitora)"),
(000,"MissingNo.(Raitora evo)"),
(000,"MissingNo.(Ponyta pre-evo)"),
(077,"Ponyta"),
(078,"Rapidash"),
(019,"Rattata"),
(020,"Raticate"),
(033,"Nidorino"),
(030,"Nidorina"),
(074,"Geodude"),
(137,"Porygon"),
(142,"Aerodactyl"),
(000,"MissingNo.(Blastoise-like Pokémon)"),
(081,"Magnemite"),
(000,"MissingNo.(Lizard Pokémon 1)"),
(000,"MissingNo.(Gorochu)"),
(004,"Charmander"),
(007,"Squirtle"),
(005,"Charmeleon"),
(008,"Wartortle"),
(006,"Charizard"),
(000,"MissingNo.(Original Wartortle evo)"),
(000,"MissingNo.(Kabutops Fossil)"),
(000,"MissingNo.(Aerodactyl Fossil)"),
(000,"MissingNo.(Pokémon Tower Ghost)"),
(043,"Oddish"),
(044,"Gloom"),
(045,"Vileplume"),
(069,"Bellsprout"),
(070,"Weepinbell"),
(071,"Victreebel"),
];