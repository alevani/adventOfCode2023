use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        let input = lines.flatten().collect();
        // part_one(input);
        part_two(input);
    }
}

/*
seeds: 79 14 55 13

seed-to-soil map:
50 98 2// osef since start is higher than max seed
52 50 48
*/

fn part_two(_: Vec<String>) {
    let mut seeds: Vec<i64> = vec![
        1367444651, 99920667, 3319921504, 153335682, 67832336, 139859832, 2322838536, 666063790,
        1591621692, 111959634, 442852010, 119609663, 733590868, 56288233, 2035874278, 85269124,
        4145746192, 55841637, 864476811, 347179760,
    ];

    // let mut seeds: Vec<i64> = vec![79, 14, 55, 13];

    // let mut seeds: Vec<i64> = vec![79, 3];
    let map = vec![
        vec![
            vec![873256303, 3438158294, 3400501],
            vec![3338810960, 408700040, 99469568],
            vec![876656804, 586381004, 55967396],
            vec![2937187724, 3352513245, 85645049],
            vec![3633224442, 4294716315, 250981],
            vec![4063203128, 3993405594, 231764168],
            vec![628606346, 884567853, 85164246],
            vec![1848085960, 2225191252, 328179324],
            vec![1686068310, 2992301693, 162017650],
            vec![1456962076, 179593806, 229106234],
            vec![0, 1520731987, 239660433],
            vec![2759350898, 1833519805, 177836826],
            vec![494634602, 642348400, 67929420],
            vec![3022832773, 758696310, 125871543],
            vec![3563677889, 4225169762, 69546553],
            vec![2637775123, 710277820, 48418490],
            vec![3148704316, 969732099, 112580498],
            vec![3261284814, 2623022420, 77526146],
            vec![489910414, 174869618, 4724188],
            vec![1187482559, 2700548566, 269479517],
            vec![713770592, 2139594657, 85596595],
            vec![850982693, 2970028083, 22273610],
            vec![932624200, 2013609609, 125985048],
            vec![799367187, 1082312597, 51615506],
            vec![3633475423, 3563677889, 429727705],
            vec![562564022, 3283192654, 66042324],
            vec![2181319395, 2553370576, 69651844],
            vec![2250971239, 1133928103, 386803884],
            vec![3438280528, 3349234978, 3278267],
            vec![2686193613, 513223719, 73157285],
            vec![239660433, 1760392420, 73127385],
            vec![487657436, 2011356631, 2252978],
            vec![312787818, 0, 174869618],
            vec![1058609248, 3154319343, 128873311],
            vec![2176265284, 508169608, 5054111],
        ],
        vec![
            vec![297032819, 3559164217, 26523093],
            vec![323555912, 2482284077, 316032053],
            vec![74171080, 3214516077, 10202585],
            vec![3176226661, 2368836568, 113447509],
            vec![2918425623, 1610395638, 257801038],
            vec![3933490965, 2171144546, 103908097],
            vec![1218064889, 55976272, 92496985],
            vec![2591090931, 0, 23819721],
            vec![1798514394, 148473257, 159960710],
            vec![2288143061, 1868196676, 302947870],
            vec![639587965, 3736262447, 6060799],
            vec![3609511269, 699586127, 40962058],
            vec![1575586870, 3249692350, 47378699],
            vec![1310561874, 2798316130, 265024996],
            vec![3650473327, 2308711439, 60125129],
            vec![1958475104, 483393073, 178493006],
            vec![259332771, 661886079, 37700048],
            vec![907741932, 980768687, 228854035],
            vec![1185908338, 23819721, 32156551],
            vec![3437918904, 3874038292, 163360770],
            vec![645648764, 3297071049, 262093168],
            vec![2648569448, 1432515231, 29635673],
            vec![1647939257, 3585687310, 150575137],
            vec![3710598456, 1209622722, 222892509],
            vec![2678205121, 740548185, 240220502],
            vec![84373665, 308433967, 174959106],
            vec![2614910652, 2275052643, 33658796],
            vec![1622965569, 3224718662, 24973688],
            vec![3289674170, 1462150904, 148244734],
            vec![3601279674, 3791635617, 8231595],
            vec![0, 3799867212, 74171080],
            vec![2136968110, 3063341126, 151174951],
            vec![1136595967, 3742323246, 49312371],
        ],
        vec![
            vec![0, 478733437, 191375707],
            vec![2494518625, 3362803490, 180386054],
            vec![1605510969, 1985802816, 27464898],
            vec![3545871802, 2267467733, 385725819],
            vec![1580307385, 1113809296, 8335179],
            vec![2267467733, 4179194655, 34953467],
            vec![1588642564, 52640953, 16868405],
            vec![768087626, 69509358, 314799711],
            vec![2835042306, 4002148814, 177045841],
            vec![320920516, 1122144475, 447167110],
            vec![1082887337, 670109144, 443700152],
            vec![3218018460, 3543189544, 156036618],
            vec![1921732119, 1569311585, 321765324],
            vec![1632975867, 384309069, 40704472],
            vec![1526587489, 425013541, 53719896],
            vec![3140697718, 2802291105, 77320742],
            vec![1726321292, 1891076909, 94725907],
            vec![3070918618, 2732512005, 69779100],
            vec![1673680339, 0, 52640953],
            vec![3374055078, 3830332090, 171816724],
            vec![2755723853, 2653193552, 79318453],
            vec![3012088147, 3242981522, 58830471],
            vec![2363412697, 3699226162, 131105928],
            vec![1821047199, 2013267714, 100684920],
            vec![2302421200, 3301811993, 60991497],
            vec![191375707, 2113952634, 129544809],
            vec![3931597621, 2879611847, 363369675],
            vec![2674904679, 4214148122, 80819174],
        ],
        vec![
            vec![3219102205, 2181622520, 201394006],
            vec![920319894, 2563844887, 124975374],
            vec![739491533, 2383016526, 180828361],
            vec![653894144, 112244681, 85597389],
            vec![3420496211, 0, 112244681],
            vec![3657404452, 197842070, 151065180],
            vec![2385949305, 1028427888, 284402820],
            vec![3532740892, 1312830708, 124663560],
            vec![379827855, 754361599, 274066289],
            vec![0, 374533744, 379827855],
            vec![1070921762, 1437494268, 744128252],
            vec![1815050014, 3237570341, 372982025],
            vec![2670352125, 2688820261, 548750080],
            vec![1045295268, 348907250, 25626494],
            vec![2188032039, 3610552366, 197917266],
        ],
        vec![
            vec![2153765789, 597465407, 100160624],
            vec![2781845200, 2181361650, 40610317],
            vec![667326513, 1345068833, 191904517],
            vec![2473693610, 3180449558, 308151590],
            vec![3613083869, 2293229230, 341401182],
            vec![1062907936, 2938916666, 34067323],
            vec![1451871003, 2221971967, 71257263],
            vec![3954485051, 4137063579, 102141117],
            vec![4192502901, 3838513492, 49627103],
            vec![1389629967, 2876675630, 62241036],
            vec![1593277643, 2972983989, 143397067],
            vec![859231030, 2634630412, 203676906],
            vec![4056626168, 790528713, 68145539],
            vec![1523128266, 2838307318, 38368312],
            vec![503645468, 3488601148, 84794865],
            vec![1333867367, 4239204696, 55762600],
            vec![2902105909, 1961894500, 219467150],
            vec![3268371481, 858674252, 266627384],
            vec![2253926413, 1125301636, 219767197],
            vec![4124771707, 3888140595, 67731194],
            vec![1159014133, 3663660258, 174853234],
            vec![3121573059, 1536973350, 56534177],
            vec![4242130004, 1593507527, 52837292],
            vec![3534998865, 697626031, 78085004],
            vec![1096975259, 503645468, 62038874],
            vec![588440333, 3116381056, 64068502],
            vec![2052224391, 3955871789, 101541398],
            vec![3178107236, 3573396013, 90264245],
            vec![1736674710, 1646344819, 315549681],
            vec![1561496578, 565684342, 31781065],
            vec![2822455517, 4057413187, 79650392],
            vec![652508835, 775711035, 14817678],
        ],
        vec![
            vec![539306376, 906765326, 12587914],
            vec![0, 164719538, 374586838],
            vec![3299714596, 2417002864, 137882274],
            vec![3574681727, 1862289721, 10695948],
            vec![1377359247, 1111480860, 147188226],
            vec![2546515862, 3738486436, 38563842],
            vec![2519543212, 3564238204, 26972650],
            vec![1619134727, 3777050278, 31686173],
            vec![551894290, 728868054, 177897272],
            vec![3834998050, 3919246788, 238635651],
            vec![729791562, 919353240, 625999],
            vec![1524547473, 3627710187, 94587254],
            vec![2399595894, 3277935618, 48769514],
            vec![1340859914, 3591210854, 36499333],
            vec![2585079704, 2011159197, 185411491],
            vec![3723551203, 3452791357, 111446847],
            vec![2895131743, 2873352765, 404582853],
            vec![374586838, 0, 164719538],
            vec![1776907125, 1258669086, 373340162],
            vec![2881001532, 2859222554, 14130211],
            vec![2217932998, 2606381854, 181662896],
            vec![2166436282, 2554885138, 51496716],
            vec![2448365408, 2788044750, 71177804],
            vec![1650820900, 3326705132, 126086225],
            vec![1110579441, 1632009248, 230280473],
            vec![3437596870, 4157882439, 137084857],
            vec![730417561, 539306376, 189561678],
            vec![4294065877, 1110579441, 901419],
            vec![2150247287, 3722297441, 16188995],
            vec![2770491195, 3808736451, 110510337],
            vec![3585377675, 1872985669, 138173528],
            vec![4073633701, 2196570688, 220432176],
        ],
        vec![
            vec![3656475570, 3037182697, 7397903],
            vec![682722270, 547529272, 780546181],
            vec![266636474, 1328075453, 316323944],
            vec![1591860664, 3642496089, 50700992],
            vec![1642561656, 266636474, 280892798],
            vec![1923454454, 1644399397, 1264527167],
            vec![3979139381, 3408096655, 6045369],
            vec![3663873473, 4002979627, 291987669],
            vec![3955861142, 3208017899, 23278239],
            vec![582960418, 3414142024, 99761852],
            vec![3187981621, 3231296138, 176800517],
            vec![3364782138, 3044580600, 163437299],
            vec![1463268451, 3513903876, 128592213],
            vec![3528219437, 2908926564, 128256133],
            vec![4175159701, 3693197081, 119807595],
            vec![3985184750, 3813004676, 189974951],
        ],
    ];
    // let map = vec![
    //     vec![vec![50, 98, 2], vec![52, 50, 48]],
    //     vec![vec![0, 15, 37], vec![37, 52, 2], vec![39, 0, 15]],
    //     vec![
    //         vec![49, 53, 8],
    //         vec![0, 11, 42],
    //         vec![42, 0, 7],
    //         vec![57, 7, 4],
    //     ],
    //     vec![vec![88, 18, 7], vec![18, 25, 70]],
    //     vec![vec![45, 77, 23], vec![81, 45, 19], vec![68, 64, 13]],
    //     vec![vec![0, 69, 1], vec![1, 0, 69]],
    //     vec![vec![60, 56, 37], vec![56, 93, 4]],
    // ];

    // let map = vec![vec![vec![10, 80, 5]]];

    for a_to_b in map {
        let mut seeds_with_new_ranges: Vec<i64> = Vec::new();
        for line in a_to_b {
            let coverage_lower_range = line[1];
            let dest = line[0];
            let coverage = line[2];

            let mut temps_seeds: Vec<i64> = Vec::new();

            while !seeds.is_empty() {
                println!("seeds before {seeds:?}");
                let chunk2 = seeds.pop().unwrap();
                let chunk1 = seeds.pop().unwrap();
                println!("seeds after {seeds:?}");

                let lower_range = chunk1;
                let upper_range = chunk1 + chunk2;

                let coverage_upper_range = coverage_lower_range + coverage - 1;

                let intersection = (
                    lower_range.max(coverage_lower_range),
                    upper_range.min(coverage_upper_range),
                );

                //  if there is an intersect between the two ranges
                if intersection.1 >= intersection.0 {
                    // all the numbers here should become the new range
                    let mapped_seeds_lower_range = intersection.0 - (coverage_lower_range - dest);
                    let mapped_seeds_upper_range = intersection.1 - (coverage_lower_range - dest);

                    if mapped_seeds_lower_range < mapped_seeds_upper_range {
                        seeds_with_new_ranges.push(mapped_seeds_lower_range);
                        seeds_with_new_ranges
                            .push(mapped_seeds_upper_range - mapped_seeds_lower_range);
                    }

                    let lower_bound_lower_range = lower_range;
                    let lower_bound_upper_range = intersection.0 - 1;

                    if lower_bound_lower_range < lower_bound_upper_range {
                        temps_seeds.push(lower_bound_lower_range);
                        temps_seeds.push(lower_bound_upper_range - lower_bound_lower_range);
                    }

                    // todo if min > max, then don't add
                    let upper_bound_lower_range = intersection.1 + 1;
                    let upper_bound_upper_range = upper_range;

                    if upper_bound_lower_range < upper_bound_upper_range {
                        temps_seeds.push(upper_bound_lower_range);
                        temps_seeds.push(upper_bound_upper_range - upper_bound_lower_range);
                    }
                } else {
                    temps_seeds.push(lower_range);
                    temps_seeds.push(upper_range - lower_range);
                    println!("temps_seeds: {:?}", temps_seeds);
                }
            }
            seeds.append(&mut temps_seeds);
            println!("temps_seeds: {:?}", temps_seeds);
        }
        seeds.append(&mut seeds_with_new_ranges);
        println!("seeds_with_new_ranges: {:?}", seeds_with_new_ranges);
        println!("seeds: {:?}", seeds);
        // panic!();
    }
    println!(
        "{:?}",
        seeds
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(_, &value)| value)
            .min()
    );
}

fn part_one(_: Vec<String>) {
    let mut seeds: Vec<i64> = vec![
        1367444651, 99920667, 3319921504, 153335682, 67832336, 139859832, 2322838536, 666063790,
        1591621692, 111959634, 442852010, 119609663, 733590868, 56288233, 2035874278, 85269124,
        4145746192, 55841637, 864476811, 347179760,
    ];
    // let mut seeds: Vec<i64> = vec![79, 14, 55, 13];
    // let mut seeds: Vec<i64> = vec![14];
    seeds.sort();

    let mut seed_max = *seeds.last().unwrap();
    let map = vec![
        vec![
            vec![873256303, 3438158294, 3400501],
            vec![3338810960, 408700040, 99469568],
            vec![876656804, 586381004, 55967396],
            vec![2937187724, 3352513245, 85645049],
            vec![3633224442, 4294716315, 250981],
            vec![4063203128, 3993405594, 231764168],
            vec![628606346, 884567853, 85164246],
            vec![1848085960, 2225191252, 328179324],
            vec![1686068310, 2992301693, 162017650],
            vec![1456962076, 179593806, 229106234],
            vec![0, 1520731987, 239660433],
            vec![2759350898, 1833519805, 177836826],
            vec![494634602, 642348400, 67929420],
            vec![3022832773, 758696310, 125871543],
            vec![3563677889, 4225169762, 69546553],
            vec![2637775123, 710277820, 48418490],
            vec![3148704316, 969732099, 112580498],
            vec![3261284814, 2623022420, 77526146],
            vec![489910414, 174869618, 4724188],
            vec![1187482559, 2700548566, 269479517],
            vec![713770592, 2139594657, 85596595],
            vec![850982693, 2970028083, 22273610],
            vec![932624200, 2013609609, 125985048],
            vec![799367187, 1082312597, 51615506],
            vec![3633475423, 3563677889, 429727705],
            vec![562564022, 3283192654, 66042324],
            vec![2181319395, 2553370576, 69651844],
            vec![2250971239, 1133928103, 386803884],
            vec![3438280528, 3349234978, 3278267],
            vec![2686193613, 513223719, 73157285],
            vec![239660433, 1760392420, 73127385],
            vec![487657436, 2011356631, 2252978],
            vec![312787818, 0, 174869618],
            vec![1058609248, 3154319343, 128873311],
            vec![2176265284, 508169608, 5054111],
        ],
        vec![
            vec![297032819, 3559164217, 26523093],
            vec![323555912, 2482284077, 316032053],
            vec![74171080, 3214516077, 10202585],
            vec![3176226661, 2368836568, 113447509],
            vec![2918425623, 1610395638, 257801038],
            vec![3933490965, 2171144546, 103908097],
            vec![1218064889, 55976272, 92496985],
            vec![2591090931, 0, 23819721],
            vec![1798514394, 148473257, 159960710],
            vec![2288143061, 1868196676, 302947870],
            vec![639587965, 3736262447, 6060799],
            vec![3609511269, 699586127, 40962058],
            vec![1575586870, 3249692350, 47378699],
            vec![1310561874, 2798316130, 265024996],
            vec![3650473327, 2308711439, 60125129],
            vec![1958475104, 483393073, 178493006],
            vec![259332771, 661886079, 37700048],
            vec![907741932, 980768687, 228854035],
            vec![1185908338, 23819721, 32156551],
            vec![3437918904, 3874038292, 163360770],
            vec![645648764, 3297071049, 262093168],
            vec![2648569448, 1432515231, 29635673],
            vec![1647939257, 3585687310, 150575137],
            vec![3710598456, 1209622722, 222892509],
            vec![2678205121, 740548185, 240220502],
            vec![84373665, 308433967, 174959106],
            vec![2614910652, 2275052643, 33658796],
            vec![1622965569, 3224718662, 24973688],
            vec![3289674170, 1462150904, 148244734],
            vec![3601279674, 3791635617, 8231595],
            vec![0, 3799867212, 74171080],
            vec![2136968110, 3063341126, 151174951],
            vec![1136595967, 3742323246, 49312371],
        ],
        vec![
            vec![0, 478733437, 191375707],
            vec![2494518625, 3362803490, 180386054],
            vec![1605510969, 1985802816, 27464898],
            vec![3545871802, 2267467733, 385725819],
            vec![1580307385, 1113809296, 8335179],
            vec![2267467733, 4179194655, 34953467],
            vec![1588642564, 52640953, 16868405],
            vec![768087626, 69509358, 314799711],
            vec![2835042306, 4002148814, 177045841],
            vec![320920516, 1122144475, 447167110],
            vec![1082887337, 670109144, 443700152],
            vec![3218018460, 3543189544, 156036618],
            vec![1921732119, 1569311585, 321765324],
            vec![1632975867, 384309069, 40704472],
            vec![1526587489, 425013541, 53719896],
            vec![3140697718, 2802291105, 77320742],
            vec![1726321292, 1891076909, 94725907],
            vec![3070918618, 2732512005, 69779100],
            vec![1673680339, 0, 52640953],
            vec![3374055078, 3830332090, 171816724],
            vec![2755723853, 2653193552, 79318453],
            vec![3012088147, 3242981522, 58830471],
            vec![2363412697, 3699226162, 131105928],
            vec![1821047199, 2013267714, 100684920],
            vec![2302421200, 3301811993, 60991497],
            vec![191375707, 2113952634, 129544809],
            vec![3931597621, 2879611847, 363369675],
            vec![2674904679, 4214148122, 80819174],
        ],
        vec![
            vec![3219102205, 2181622520, 201394006],
            vec![920319894, 2563844887, 124975374],
            vec![739491533, 2383016526, 180828361],
            vec![653894144, 112244681, 85597389],
            vec![3420496211, 0, 112244681],
            vec![3657404452, 197842070, 151065180],
            vec![2385949305, 1028427888, 284402820],
            vec![3532740892, 1312830708, 124663560],
            vec![379827855, 754361599, 274066289],
            vec![0, 374533744, 379827855],
            vec![1070921762, 1437494268, 744128252],
            vec![1815050014, 3237570341, 372982025],
            vec![2670352125, 2688820261, 548750080],
            vec![1045295268, 348907250, 25626494],
            vec![2188032039, 3610552366, 197917266],
        ],
        vec![
            vec![2153765789, 597465407, 100160624],
            vec![2781845200, 2181361650, 40610317],
            vec![667326513, 1345068833, 191904517],
            vec![2473693610, 3180449558, 308151590],
            vec![3613083869, 2293229230, 341401182],
            vec![1062907936, 2938916666, 34067323],
            vec![1451871003, 2221971967, 71257263],
            vec![3954485051, 4137063579, 102141117],
            vec![4192502901, 3838513492, 49627103],
            vec![1389629967, 2876675630, 62241036],
            vec![1593277643, 2972983989, 143397067],
            vec![859231030, 2634630412, 203676906],
            vec![4056626168, 790528713, 68145539],
            vec![1523128266, 2838307318, 38368312],
            vec![503645468, 3488601148, 84794865],
            vec![1333867367, 4239204696, 55762600],
            vec![2902105909, 1961894500, 219467150],
            vec![3268371481, 858674252, 266627384],
            vec![2253926413, 1125301636, 219767197],
            vec![4124771707, 3888140595, 67731194],
            vec![1159014133, 3663660258, 174853234],
            vec![3121573059, 1536973350, 56534177],
            vec![4242130004, 1593507527, 52837292],
            vec![3534998865, 697626031, 78085004],
            vec![1096975259, 503645468, 62038874],
            vec![588440333, 3116381056, 64068502],
            vec![2052224391, 3955871789, 101541398],
            vec![3178107236, 3573396013, 90264245],
            vec![1736674710, 1646344819, 315549681],
            vec![1561496578, 565684342, 31781065],
            vec![2822455517, 4057413187, 79650392],
            vec![652508835, 775711035, 14817678],
        ],
        vec![
            vec![539306376, 906765326, 12587914],
            vec![0, 164719538, 374586838],
            vec![3299714596, 2417002864, 137882274],
            vec![3574681727, 1862289721, 10695948],
            vec![1377359247, 1111480860, 147188226],
            vec![2546515862, 3738486436, 38563842],
            vec![2519543212, 3564238204, 26972650],
            vec![1619134727, 3777050278, 31686173],
            vec![551894290, 728868054, 177897272],
            vec![3834998050, 3919246788, 238635651],
            vec![729791562, 919353240, 625999],
            vec![1524547473, 3627710187, 94587254],
            vec![2399595894, 3277935618, 48769514],
            vec![1340859914, 3591210854, 36499333],
            vec![2585079704, 2011159197, 185411491],
            vec![3723551203, 3452791357, 111446847],
            vec![2895131743, 2873352765, 404582853],
            vec![374586838, 0, 164719538],
            vec![1776907125, 1258669086, 373340162],
            vec![2881001532, 2859222554, 14130211],
            vec![2217932998, 2606381854, 181662896],
            vec![2166436282, 2554885138, 51496716],
            vec![2448365408, 2788044750, 71177804],
            vec![1650820900, 3326705132, 126086225],
            vec![1110579441, 1632009248, 230280473],
            vec![3437596870, 4157882439, 137084857],
            vec![730417561, 539306376, 189561678],
            vec![4294065877, 1110579441, 901419],
            vec![2150247287, 3722297441, 16188995],
            vec![2770491195, 3808736451, 110510337],
            vec![3585377675, 1872985669, 138173528],
            vec![4073633701, 2196570688, 220432176],
        ],
        vec![
            vec![3656475570, 3037182697, 7397903],
            vec![682722270, 547529272, 780546181],
            vec![266636474, 1328075453, 316323944],
            vec![1591860664, 3642496089, 50700992],
            vec![1642561656, 266636474, 280892798],
            vec![1923454454, 1644399397, 1264527167],
            vec![3979139381, 3408096655, 6045369],
            vec![3663873473, 4002979627, 291987669],
            vec![3955861142, 3208017899, 23278239],
            vec![582960418, 3414142024, 99761852],
            vec![3187981621, 3231296138, 176800517],
            vec![3364782138, 3044580600, 163437299],
            vec![1463268451, 3513903876, 128592213],
            vec![3528219437, 2908926564, 128256133],
            vec![4175159701, 3693197081, 119807595],
            vec![3985184750, 3813004676, 189974951],
        ],
    ];
    // let map = vec![
    //     vec![vec![50, 98, 2], vec![52, 50, 48]],
    //     vec![vec![0, 15, 37], vec![37, 52, 2], vec![39, 0, 15]],
    //     vec![
    //         vec![49, 53, 8],
    //         vec![0, 11, 42],
    //         vec![42, 0, 7],
    //         vec![57, 7, 4],
    //     ],
    //     vec![vec![88, 18, 7], vec![18, 25, 70]],
    //     vec![vec![45, 77, 23], vec![81, 45, 19], vec![68, 64, 13]],
    //     vec![vec![0, 69, 1], vec![1, 0, 69]],
    //     vec![vec![60, 56, 37], vec![56, 93, 4]],
    // ];

    for a_to_b in map {
        let mut comparator = seeds.clone();
        for line in a_to_b {
            let start = line[1];
            let dest = line[0];
            let coverage = line[2];

            if start > seed_max {
                continue;
            }

            for (index, seed) in seeds.clone().iter().enumerate() {
                if seed >= &start
                    && seed <= &(start + coverage - 1)
                    && seeds[index] == comparator[index]
                {
                    comparator[index] += dest - start
                }
            }
            seed_max = *seeds.iter().max().unwrap(); // yikes
        }
        seeds = comparator;
    }

    println!("{:?}", seeds.iter().min());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
