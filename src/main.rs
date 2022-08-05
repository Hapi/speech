use rand::Rng;

const MAX_NUMBER_OF_PHRASE_COLUMNS: usize = 5;
const MAX_NUMBER_OF_PHRASE_ROWS: usize = 10;
const MAX_NUMBER_OF_SENTENCES: usize = 10;

fn main() {
    let phrases: [[&str; MAX_NUMBER_OF_PHRASE_ROWS]; MAX_NUMBER_OF_PHRASE_COLUMNS] = [
        [
            "eduskuntaryhmämme",
            "hallituksen",
            "valiokunnan muistion ",
            "kaiken tämän keskellä ei sovi unohtaa, että edellisen hallituksen",
            "niinpä työnantajien ja eri etujärjestöjen",
            "myös ministeriön",
            "kuntasektorin toimintaedellytyksiä tarkastelleen työryhmän",
            "toisaalta julkisen terveydenhuollon",
            "Euroopan unioninkin",
            "näiden strategioiden",
        ],
        [
            "linjausten",
            "talouspoliittisten esitysten",
            "erityisvaatimusten",
            "keskeisten tavoitteiden",
            "ehdottamien valtionosuuksien tarkistamisen",
            "vahvojen tahdonilmaisujen",
            "esittämien menokehysten",
            "ohjelman vaatimusten",
            "edellyttämän yhteistyön",
            "rakenteellisten uudistusten",
        ],
        [
            "etupainotteisuus",
            "tarkka, syvällinen ja yksityiskohtainen pohdinta",
            "lieventäminen",
            "turvaaminen verouudistuksessa",
            "kohentaminen ja säilyttäminen",
            "ratkaiseminen hallitusohjelmassa esitetyllä tavalla",
            "analyyttinen selvittäminen",
            "esille nostaminen",
            "sivuuttaminen sen sijaan",
            "ennakkoluulottomuus, etten sanoisi uskaliaisuus,",
        ],
        [
            "on ratkaisu rakennetyöttömyyden ongelmiin",
            "heijastuu kansainväliseen kilpailukykyymme",
            "tasapainottaa perusrahoituksen haasteita",
            "johtaa tuloveroasteen nousuun ja yleiskustannusten laskuun",
            "on tärkeää eläkeläisten ja vanhusten aseman parantamiseksi",
            "turvaa palvelurakenteet ja perusparannustarpeet",
            "on edellytys perusterveydenhuollon kehittämishankkeiden onnistumiselle",
            "pitää yllä kulutuskysyntää ja työllisyyskehitystä",
            "näkyy opiskelijoiden ja lapsiperheiden tulonsiirtoina",
            "hoidetaan monipuolisella työttömien täsmäkoulutuksella",
        ],
        [
            "eduskunnassa tehdyn aloitteen mukaisesti",
            "pitkällä aikavälillä",
            "eikä lisää ratkaisevasti valtiontalouden alijäämää",
            "hyvinvointiyhteiskunnalle ominaisella tavalla",
            "ja on myös edellytys yhteisverokannan laajentamiselle",
            "sekä vähentää yritysten tarvetta laajoihin irtisanomisiin",
            "mahdollistaen hyvinvointipalveluiden ylläpitämisen",
            "erityisesti pitkän laskusuhdanteen aikana",
            "peruspalvelurakenteita unohtamatta",
            "dynaamisesti ja pienpalkkaaloja kannustavalla tavalla",
        ],
        ];

    let mut rng = rand::thread_rng();
    for _ in 0..MAX_NUMBER_OF_SENTENCES {
        for col in 0..MAX_NUMBER_OF_PHRASE_COLUMNS {
            let row = rng.gen_range(0..MAX_NUMBER_OF_PHRASE_ROWS);
            let phrase = phrases[col][row];
            print!("{}", if col == 0 { capitalise(&phrase) } else { phrase.to_string() });
            print!("{}", if col == MAX_NUMBER_OF_PHRASE_COLUMNS - 1 { "." } else { " " });
        }
        print!(" ");
    }
    println!("");
}

fn capitalise(sentence: &str) -> String {
    sentence[0..1].to_uppercase() + &sentence[1..]
}
