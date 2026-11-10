// ./src/feature/id_kinship.rs

/*
 * # LITEROWY ZAPIS ŚCIEŻKI RELACJI
 *
 * ZASADA
 *   Ścieżkę czyta się od lewej do prawej, litera po literze; każda litera to jeden krok.
 *   np. PD – dziecko ojca (rodzeństwo przyrodnie ojczyste), MS – siostra matki.
 *
 * ## System "1" - Główny
 *
 * ### GRUPY LITER WEDŁUG KSZTAŁTU
 *
 * * męska    – pionowa kreska po lewej z poprzeczką w połowie wysokości (lub jej zaczątkiem)
 *              E, P, F, B, H, R, A
 * * żeńska   – skos z lewego górnego punktu, do prawego dolnego.
 *              X, M, N, S, W, K, Y
 * * nijaka   – obłość
 *              U, O, D, G, C, Q, J
 * * bez płci – trzon lub poprzeczki, bez cech płci; służą jako modyfikatory
 *              I, T, Z, L
 *
 * ### LITERY RELACJI
 *
 *               męska           żeńska           nijaka
 *   osoba       E – on          X – ona          U – unisex/Oboje
 *   rodzice     P – ojciec      M – matka        O – rodzic/rodzice
 *   dzieci      F – syn         N – córka        D – dziecko/dzieci
 *   rodzeństwo  B – brat        S – siostra      G – brat/siostra//rodzeństwo
 *   małżonkowie H – mąż         W – żona         C – małżonek
 *   przyjaciele A – przyjaciel  Y – przyjaciółka J – przyjaciel/przyjaciółka
 *   krewni      R – krewny      K – krewna       Q – krewny/krewna
 *               (krewny o nieustalonej ścieżce: z metryki lub dopasowania DNA)
 *
 *     Pochodzenie liter:
 *      E Ego, X chromosomy XX (i optyczna para z E), U Unisex,
 *      P Pater, M Mater, O Origin / Origo,
 *      F Filius, N nata (łac. córka), D Descendant,
 *      B Brother / Bruder, S Sister / Schwester, G Generation / Geschwister,
 *      H Husband, W Wife, C Couples,
 *      A amicus (łac. przyjaciel), Y yedidah (hebr. przyjaciółka) i podobne do odwróconego A,
 *      J Jedidah – biblijna pisownia tego samego słowa; rdzeń yadad (kochać), bez rodzaju, i połowa z U
 *      R Relative, K kin / kinswoman (ang. krewna) i podobne do K,
 *      Q qarīb / qarībah / aqārib (arab. krewny / krewna / krewni), oraz Query - szukane.
 *
 * ### MODYFIKATORY (stoją przed literą relacji)
 *
 * Modyfikator zmienia rodzaj więzi, a rolę zostawia literze po nim.
 *
 *   Z – przybrane, adopcja, przyszywane   (niem. Zieh-: Ziehvater, Ziehkind)
 *   L – chrzestne                          (łac. levantes – chrzestni w metrykach)
 *   T – świadek                            (łac. testis, testes – świadkowie w metrykach)
 *   I – relacja intymna                    (łac. intimus)
 *   [] – szczególny modyfikator gdy trzeba wydłużyć zakres modyfikatora
 *
 * ### PRZYKŁADY
 *   ZP   – przybrany ojciec              LP   – ojciec chrzestny
 *   ZF   – adoptowany syn                LM   – matka chrzestna
 *   ZS   – przybrana siostra             LF   – syn chrzestny
 *   MZS  – przyszywana siostra matki     LN   – córka chrzestna
 *   TE   – świadek (on)                  IY   – kochanka
 *   TX   – świadek (ona)                 IA   – kochanek
 *   TA   – świadek, przyjaciel           BA   – przyjaciel brata
 *   MR   – nieustalony krewny matki      WB   – brat żony
 *
 *  ## System "2" - potrzebny dla (wybiurczo-uciętej) kompatybilności z pewnym słownikiem
 *
 * P – rodzic, F – ojciec, M – matka,
 * C – dziecko, S – syn, D – córka,
 * G – rodzeństwo, B – brat, Z – siostra,
 * E – małżonek, H – mąż, W – żona.
 * Znak „+” po symbolu oznacza „starszy”, znak „−” po symbolu oznacza „młodszy”.
 *
 */

enum KinshipNotation {}
