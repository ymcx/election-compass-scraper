const COUNTY = {'AP': [3.0, 2.0, 1.0, 2.0, 2.0, 2.0, 2.0, 3.0, 2.0, 0.0, 2.0, 2.0, 3.0, 2.0, 0.0, 2.0, 2.0, 2.0, 3.0, 1.0, 2.0, 3.0, 2.0, 2.0, 3.0], 'EOP': [2.7, 2.6, 0.2, 0.4, 0.8, 1.4, 2.2, 2.8, 2.9, 0.1, 2.8, 2.4, 2.9, 1.8, 0.4, 0.7, 0.9, 2.8, 3.0, 2.8, 2.7, 0.9, 0.4, 2.8, 3.0], 'KD': [2.0251046025104604, 1.4163179916317992, 1.8702928870292888, 1.4811715481171548, 1.294979079497908, 1.3744769874476988, 1.49163179916318, 2.564853556485356, 1.0753138075313808, 1.3096234309623431, 0.9456066945606695, 1.2238493723849373, 2.3535564853556483, 2.3179916317991633, 1.0481171548117154, 1.5460251046025104, 1.2573221757322175, 1.6966527196652719, 1.6882845188284519, 2.422594142259414, 0.6631799163179917, 1.4330543933054394, 1.0418410041841004, 1.8368200836820083, 1.2405857740585775], 'Kesk.': [2.3089108910891087, 1.4732673267326732, 1.5247524752475248, 1.494059405940594, 1.1356435643564355, 1.3752475247524751, 1.798019801980198, 2.503960396039604, 1.2326732673267327, 0.7495049504950495, 1.7772277227722773, 1.1, 2.386138613861386, 2.3366336633663365, 1.2742574257425743, 1.4148514851485148, 0.903960396039604, 1.7643564356435644, 1.9128712871287128, 2.6633663366336635, 0.8960396039603961, 0.7475247524752475, 0.5465346534653466, 1.7871287128712872, 1.5475247524752476], 'Kok.': [1.9774436090225564, 1.0868838763575606, 2.2623224728487887, 1.962406015037594, 1.608187134502924, 1.345029239766082, 1.9423558897243107, 2.4335839598997495, 0.9983291562238931, 0.7936507936507936, 1.6758563074352548, 0.9281537176274018, 2.3015873015873014, 2.117794486215539, 1.5279866332497911, 1.509607351712615, 1.6624895572263994, 1.2614870509607352, 1.960735171261487, 2.2681704260651627, 0.36006683375104426, 1.9289891395154553, 1.5196324143692566, 1.77109440267335, 1.5730994152046784], 'KriP': [2.5, 2.0, 1.8333333333333333, 0.6666666666666666, 0.8333333333333334, 2.1666666666666665, 0.8333333333333334, 2.3333333333333335, 0.5, 0.6666666666666666, 0.8333333333333334, 1.6666666666666667, 2.5, 2.0, 0.6666666666666666, 2.0, 0.8333333333333334, 2.8333333333333335, 2.0, 3.0, 0.6666666666666666, 1.6666666666666667, 0.16666666666666666, 2.5, 0.8333333333333334], 'Lib.': [2.108695652173913, 1.108695652173913, 1.6956521739130435, 1.7826086956521738, 1.8695652173913044, 1.7173913043478262, 2.0652173913043477, 2.347826086956522, 1.0217391304347827, 0.8478260869565217, 1.934782608695652, 0.7391304347826086, 2.1739130434782608, 1.7608695652173914, 1.6304347826086956, 1.391304347826087, 2.369565217391304, 1.3478260869565217, 2.5434782608695654, 1.9565217391304348, 1.9565217391304348, 2.608695652173913, 1.6521739130434783, 2.4130434782608696, 1.9130434782608696], 'Liik.': [2.418918918918919, 1.5675675675675675, 1.9324324324324325, 1.1756756756756757, 1.1486486486486487, 1.5135135135135136, 1.472972972972973, 2.324324324324324, 0.8918918918918919, 0.6756756756756757, 1.6756756756756757, 1.4189189189189189, 2.5945945945945947, 2.418918918918919, 0.8918918918918919, 1.527027027027027, 1.2972972972972974, 1.5405405405405406, 1.9594594594594594, 2.581081081081081, 0.9459459459459459, 2.0945945945945947, 0.9459459459459459, 2.472972972972973, 1.4189189189189189], 'PS': [2.1299559471365637, 1.5220264317180616, 1.8568281938325992, 1.578193832599119, 1.236784140969163, 1.4151982378854626, 0.7323788546255506, 2.7334801762114536, 0.21806167400881057, 1.36784140969163, 0.6013215859030837, 0.6696035242290749, 2.246696035242291, 2.3127753303964758, 1.051762114537445, 1.5814977973568283, 1.3204845814977975, 1.7984581497797356, 1.3898678414096917, 2.5715859030837005, 0.3711453744493392, 1.696035242290749, 1.3094713656387664, 2.1277533039647576, 0.538546255506608], 'RKP': [2.377049180327869, 1.4631147540983607, 1.709016393442623, 1.1926229508196722, 1.1352459016393444, 1.4549180327868851, 2.209016393442623, 2.651639344262295, 2.0491803278688523, 0.4098360655737705, 2.1639344262295084, 2.372950819672131, 2.6188524590163933, 2.319672131147541, 1.0, 1.110655737704918, 1.1967213114754098, 1.6270491803278688, 2.1434426229508197, 2.6844262295081966, 1.1024590163934427, 0.8565573770491803, 0.9672131147540983, 1.7459016393442623, 2.0737704918032787], 'SDP': [2.7515151515151515, 2.1333333333333333, 1.0545454545454545, 1.3298701298701299, 1.3082251082251082, 1.2961038961038962, 1.7515151515151515, 2.535930735930736, 1.877056277056277, 0.3212121212121212, 2.2510822510822512, 1.4025974025974026, 2.6346320346320344, 2.516017316017316, 1.1454545454545455, 1.0571428571428572, 0.9385281385281385, 1.9688311688311688, 2.4060606060606062, 2.50995670995671, 1.1861471861471862, 1.0311688311688312, 0.4857142857142857, 1.9316017316017315, 2.0952380952380953], 'SKP': [2.764705882352941, 3.0, 0.4117647058823529, 0.7647058823529411, 1.0, 1.6470588235294117, 1.7058823529411764, 2.4705882352941178, 2.6470588235294117, 0.0, 2.764705882352941, 2.2941176470588234, 2.823529411764706, 2.8823529411764706, 0.35294117647058826, 0.47058823529411764, 0.17647058823529413, 2.9411764705882355, 2.8823529411764706, 2.7058823529411766, 2.6470588235294117, 0.8235294117647058, 0.23529411764705882, 2.6470588235294117, 2.7058823529411766], 'Sit': [2.3304347826086955, 1.791304347826087, 1.5043478260869565, 1.626086956521739, 1.2347826086956522, 1.3478260869565217, 1.617391304347826, 2.5043478260869567, 1.4260869565217391, 0.6260869565217392, 1.7739130434782608, 1.0869565217391304, 2.5130434782608697, 2.4869565217391303, 0.8782608695652174, 1.2869565217391303, 0.991304347826087, 1.8695652173913044, 2.0521739130434784, 2.582608695652174, 1.0695652173913044, 1.1565217391304348, 0.5826086956521739, 2.234782608695652, 1.6521739130434783], 'VKK': [2.3076923076923075, 2.0, 1.0769230769230769, 0.6923076923076923, 1.2307692307692308, 1.7692307692307692, 0.5384615384615384, 2.6153846153846154, 0.6153846153846154, 1.6923076923076923, 0.15384615384615385, 0.6153846153846154, 2.6923076923076925, 2.3076923076923075, 0.38461538461538464, 2.076923076923077, 1.2307692307692308, 2.3846153846153846, 1.3076923076923077, 2.5384615384615383, 0.6923076923076923, 0.9230769230769231, 0.6153846153846154, 2.1538461538461537, 0.15384615384615385], 'VL': [2.491228070175439, 2.0, 1.4736842105263157, 0.8771929824561403, 0.8245614035087719, 1.6491228070175439, 0.543859649122807, 2.5964912280701755, 0.38596491228070173, 1.543859649122807, 0.2982456140350877, 1.0175438596491229, 2.4210526315789473, 2.43859649122807, 0.8245614035087719, 1.4035087719298245, 1.2456140350877194, 2.2982456140350878, 1.4385964912280702, 2.6315789473684212, 0.5263157894736842, 0.9298245614035088, 0.5614035087719298, 2.3508771929824563, 0.2631578947368421], 'Vas.': [2.741496598639456, 2.621315192743764, 0.7052154195011338, 1.1156462585034013, 1.0963718820861679, 1.354875283446712, 1.6848072562358276, 2.463718820861678, 2.4342403628117912, 0.14512471655328799, 2.6927437641723357, 1.6360544217687074, 2.759637188208617, 2.683673469387755, 0.7664399092970522, 0.8197278911564626, 0.7233560090702947, 2.5861678004535147, 2.766439909297052, 2.6031746031746033, 2.201814058956916, 0.8786848072562359, 0.2494331065759637, 2.2176870748299318, 2.561224489795918], 'Vihr.': [2.511392405063291, 2.0746835443037974, 1.1367088607594937, 1.4240506329113924, 1.4949367088607595, 1.3075949367088608, 2.110126582278481, 2.348101265822785, 2.310126582278481, 0.17468354430379746, 2.6860759493670887, 1.388607594936709, 2.730379746835443, 2.4329113924050634, 1.1924050632911392, 0.970886075949367, 1.1759493670886076, 2.1215189873417724, 2.7265822784810125, 2.448101265822785, 2.1050632911392406, 1.440506329113924, 0.5177215189873418, 2.239240506329114, 2.712658227848101]}
const MUNICIPAL = {'AP': [3.0, 0.0, 3.0, 3.0, 2.0, 3.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 2.0, 3.0, 3.0, 0.0, 3.0, 0.0, 3.0, 0.0, 3.0], 'EOP': [2.9, 0.5, 2.3, 1.7, 0.6, 3.0, 0.3, 0.5, 0.5, 1.2, 0.2, 0.5, 1.3, 3.0, 0.0, 0.0, 2.9, 2.5, 2.8, 0.2, 2.7, 0.2, 2.3, 0.2, 2.7], 'KD': [2.3629283489096573, 1.1666666666666667, 1.7149532710280373, 2.454828660436137, 1.7959501557632398, 0.5794392523364486, 2.858255451713396, 1.9735202492211839, 1.6713395638629283, 1.719626168224299, 1.0623052959501558, 1.222741433021807, 0.9719626168224299, 1.691588785046729, 1.0155763239875388, 1.3395638629283488, 0.8177570093457944, 2.0514018691588785, 1.8566978193146417, 0.9080996884735203, 1.4781931464174456, 1.0264797507788161, 0.9750778816199377, 1.1993769470404985, 0.19158878504672897], 'Kesk.': [2.2324420677361854, 1.0570409982174689, 1.5244206773618538, 2.0081996434937612, 1.602139037433155, 0.6417112299465241, 2.1597147950089126, 1.9026737967914438, 1.7550802139037434, 1.6998217468805703, 0.9825311942959002, 0.9294117647058824, 0.9194295900178253, 1.4541889483065953, 1.0274509803921568, 1.3714795008912657, 1.0627450980392157, 2.101247771836007, 2.0427807486631018, 1.003921568627451, 1.7272727272727273, 0.9543672014260249, 1.0748663101604279, 1.1654188948306596, 1.2035650623885918], 'Kok.': [2.22870249017038, 1.5897771952817825, 1.7100262123197902, 1.5802752293577982, 1.1041939711664481, 0.8496068152031455, 1.990170380078637, 2.4436435124508518, 2.1815203145478375, 1.194954128440367, 0.9442988204456094, 1.3443643512450851, 1.059305373525557, 1.7332896461336829, 1.1159895150720838, 1.4642857142857142, 1.1415465268676277, 1.8328964613368284, 2.0022935779816513, 1.081913499344692, 1.6051769331585846, 1.0232634338138926, 1.042267365661861, 1.3224115334207078, 1.3938401048492792], 'KriP': [2.3333333333333335, 0.3333333333333333, 1.0, 2.6666666666666665, 2.0, 1.0, 2.3333333333333335, 2.3333333333333335, 1.0, 2.3333333333333335, 0.6666666666666666, 1.6666666666666667, 1.0, 2.0, 0.6666666666666666, 1.3333333333333333, 0.0, 3.0, 0.0, 2.6666666666666665, 1.0, 2.0, 1.6666666666666667, 0.3333333333333333, 0.0], 'Lib.': [2.0217391304347827, 1.923913043478261, 2.108695652173913, 0.9239130434782609, 0.5434782608695652, 1.6195652173913044, 0.6413043478260869, 2.717391304347826, 2.097826086956522, 0.5869565217391305, 0.3804347826086957, 2.0543478260869565, 0.7391304347826086, 2.4130434782608696, 1.0869565217391304, 1.141304347826087, 1.75, 1.7391304347826086, 2.2282608695652173, 0.9021739130434783, 1.2173913043478262, 0.9891304347826086, 0.6847826086956522, 1.5869565217391304, 2.217391304347826], 'Liik.': [2.4434782608695653, 1.1130434782608696, 1.7739130434782608, 1.7217391304347827, 1.2869565217391303, 0.6956521739130435, 1.982608695652174, 2.234782608695652, 1.9217391304347826, 1.4434782608695653, 1.0, 1.4434782608695653, 0.8347826086956521, 1.791304347826087, 1.0782608695652174, 1.382608695652174, 0.9565217391304348, 2.1565217391304348, 1.8782608695652174, 1.1478260869565218, 1.608695652173913, 1.1304347826086956, 1.2521739130434784, 1.0782608695652174, 1.2782608695652173], 'PS': [2.38895486935867, 1.0552256532066508, 1.0629453681710215, 1.8729216152019001, 1.4394299287410925, 0.20368171021377673, 2.398456057007126, 2.3865795724465557, 1.9121140142517814, 1.832541567695962, 0.8735154394299287, 1.3580760095011877, 0.8687648456057007, 1.4637767220902613, 1.1656769596199525, 1.7903800475059382, 0.23634204275534443, 1.9809976247030878, 0.8188836104513064, 2.3307600950118763, 0.7351543942992874, 2.1549881235154396, 0.5944180522565321, 1.1959619952494063, 0.22268408551068883], 'RKP': [2.305317324185249, 1.2178387650085762, 2.783876500857633, 1.6740994854202402, 1.137221269296741, 1.1012006861063466, 1.797598627787307, 1.993138936535163, 1.8970840480274442, 1.7941680960548885, 0.9759862778730704, 1.248713550600343, 0.8867924528301887, 1.946826758147513, 0.9056603773584906, 0.9656946826758147, 1.8181818181818181, 2.240137221269297, 2.4957118353344767, 0.38250428816466553, 2.1989708404802744, 0.4614065180102916, 1.3945111492281304, 1.0823327615780447, 2.2847341337907374], 'SDP': [2.5734635734635734, 1.2201872201872201, 1.8437118437118436, 1.6796906796906796, 1.2067562067562068, 1.256003256003256, 1.5836385836385836, 1.3406593406593406, 1.199023199023199, 1.9975579975579976, 0.9454619454619455, 0.9312169312169312, 1.312169312169312, 2.2205942205942204, 0.8624338624338624, 1.1111111111111112, 1.663003663003663, 2.3382173382173383, 2.230769230769231, 0.6288156288156288, 2.114774114774115, 0.628001628001628, 1.5836385836385836, 0.8974358974358975, 2.167684167684168], 'SKP': [2.8636363636363638, 0.3181818181818182, 2.3636363636363638, 2.0454545454545454, 0.9545454545454546, 2.3636363636363638, 0.09090909090909091, 0.6818181818181818, 0.22727272727272727, 2.6818181818181817, 0.36363636363636365, 0.45454545454545453, 1.6818181818181819, 2.8636363636363638, 0.3181818181818182, 0.45454545454545453, 2.590909090909091, 2.8636363636363638, 2.772727272727273, 0.13636363636363635, 2.727272727272727, 0.18181818181818182, 2.409090909090909, 0.36363636363636365, 2.727272727272727], 'Sit': [2.324773413897281, 1.0090634441087614, 1.551359516616314, 1.9501510574018126, 1.445619335347432, 0.9244712990936556, 1.7613293051359518, 1.8021148036253776, 1.542296072507553, 1.7734138972809668, 0.7885196374622356, 0.9486404833836858, 0.918429003021148, 2.009063444108761, 0.7794561933534743, 1.0921450151057401, 1.1042296072507554, 2.1540785498489425, 1.9365558912386707, 1.0981873111782476, 1.702416918429003, 1.0740181268882176, 1.13595166163142, 1.0332326283987916, 1.404833836858006], 'VKK': [2.4, 0.2, 0.3, 2.5, 1.8, 0.2, 2.6, 2.5, 1.6, 2.4, 0.8, 1.3, 0.2, 1.6, 1.3, 1.2, 0.0, 2.2, 1.2, 2.4, 1.2, 2.1, 1.3, 0.7, 0.3], 'VL': [2.459016393442623, 0.6885245901639344, 0.8852459016393442, 2.1311475409836067, 1.3770491803278688, 0.18032786885245902, 2.2459016393442623, 2.2950819672131146, 1.819672131147541, 2.0655737704918034, 0.7868852459016393, 1.5245901639344261, 0.47540983606557374, 1.639344262295082, 1.1475409836065573, 1.6065573770491803, 0.13114754098360656, 2.1475409836065573, 0.5901639344262295, 2.459016393442623, 0.6229508196721312, 2.278688524590164, 0.7704918032786885, 1.1311475409836065, 0.14754098360655737], 'Vas.': [2.713301171605789, 0.7822191592005513, 2.065472088215024, 1.768435561681599, 1.0537560303239146, 2.008959338387319, 0.7760165403170227, 0.7201929703652653, 0.6753962784286699, 2.311509303928325, 0.5499655410062027, 0.715368711233632, 1.388697450034459, 2.679531357684356, 0.4093728463128877, 0.6657477601654032, 2.2598208132322535, 2.6636802205375605, 2.5864920744314266, 0.2529290144727774, 2.5320468642315643, 0.27636113025499653, 2.062026188835286, 0.5079255685733977, 2.66092350103377], 'Vihr.': [2.6275579809004093, 1.0620736698499318, 2.136425648021828, 1.4679399727148703, 0.8587994542974079, 2.3744884038199183, 0.9474761255115962, 1.1323328785811733, 1.3867667121418827, 1.7339699863574352, 0.4311050477489768, 1.0893587994542975, 1.1678035470668486, 2.869031377899045, 0.30286493860845837, 0.4495225102319236, 2.5068212824010914, 2.4822646657571625, 2.65075034106412, 0.18894952251023192, 2.489085948158254, 0.2646657571623465, 1.8533424283765347, 0.8390177353342428, 2.7633015006821284]}
