fn code_7() {

    let mut input1 = String::new();

    let mut file:i32 = input1.trim().parse().expect("invalid input");

    match file {
        a => {
            let consulting = "Services offered: Analytics consulting services
                                                \nCustomer experience
                                                \nCybersecurity, stategy, risk, complaince and resilience
                                                \nDigital transformation
                                                \nRisk consilting services
                                                \nSupply chain and operations
                                                \nTechnology transformation";
        let mut new_file = std::fs::file::create("Aigbona_Juliet.txt").expect("create failed");
        new_file.write_all("Name: Aigbona Juliet\n".as_bytes()).expect("write failed");
        new_file.write_all("Department: Consulting\n".as_bytes()).expect("write failed");
        new_file.write_all(consulting.as_bytes()).expect("write failed");
     }


     b => {
        let assurance = {"Services offered : Audit services
                                                \nClimate chnage and sustainability services
                                                \nFinancial accounting advisory services
                                                \nForensic and integrity services
                                                \nPrivate client audit experience
                                                \nAccounting Link
                                                \nAssurance"};
        let mut new_file = std::fs::file::create("akpevwe_iloka.txt").expect("create failed");
        new_file.write_all("Name: Akpevwe Iloka\n".as_bytes()).expect("write failed");
        new_file.write_all("Department: Assuance\n".as_bytes()).expect("write failed");
        new_file.write_all(assurance.as_bytes()).expect("write failed");
     }
     _ => println!("Invalid selection");
}

fn code_8() {
     let mut input1 = String::new();

    let mut file:i32 = input1.trim().parse().expect("invalid input");

    match file {
        c => {
            let People_and_workforce = {"Services offered: Change management and experience
                                                            \nHR transformation
                                                            \nintergrated workforce mobility
                                                            \nlearning and development consulting
                                                            \nRecognition and reward advisory
                                                            \nWorkforce analytics
                                                            \nPeople and workforce "};
        let mut new_file = std::fs::file::create("gbenga_daniels.txt").expect("create failed");
        new_file.write_all("Name: Gbenga Daniels\n".as_bytes()).expect("write failed");
        new_file.write_all("Department: People and workforce\n".as_bytes()).expect("write failed");
        new_file.write_all(People_and_workforce.as_bytes()).expect("write failed");

     }

     d => {
        let tax = {"Services offered: Tax planning
                                        \nTax function operations
                                        \nTax policy and controversy
                                        \nGlobal trade
                                        \nTax accounting
                                        \nTAx compliance
                                        \nTransaction tax"};
        let mut new_file = std::fs::file::create("adamu_sagamu.txt").expect("create failed");
        new_file.write_all("Name: Adamu Sagamu\n".as_bytes()).expect("write failed");
        new_file.write_all("Department: Tax\n".as_bytes()).expect("write failed");
        new_file.write_all(Tax.as_bytes()).expect("write failed");
     }
     _ => println!("Invalid selection");
}

fn code_9() {
    let mut input1 = String::new();

    let mut file:i32 = input1.trim().parse().expect("invalid input");

    match file {
        e => {
            let Strategy = {"Services offered: Strategy consulting
                            \nCorporate and growth strategy
                            \nTransaction strategy and execution
                            \nRestructuring and turnaround strategy
                            \nIndustry strategy
                            \nDigital business building
                            \nCommercial strategy"};
        let mut new_file = std::fs::file::create("ehis_oro.txt").expect("create failed");
        new_file.write_all("Name: Ehis Ero\n".as_bytes()).expect("write failed");
        new_file.write_all("Department: Strategy by EY-Parthenon\n".as_bytes()).expect("write failed");
        new_file.write_all(Strategy.as_bytes()).expect("write failed");

    f => {
        let Transactions_and_corporate_finance = {"Services offered: Corporate finance
                                                    \nDivestments and carve-outs
                                                    \nSustainability and ESG services
                                                    \nM & A advisory
                                                    \nM & A intergration
                                                    \nM & A technology and tools
                                                    \nM & A advanced analytics"};
        let mut new_file = std::fs::file::create("maria_akinsola.txt").expect("create failed");
        new_file.write_all("Name: Maria Akinsola\n".as_bytes()).expect("write failed");
        new_file.write_all("Department: Transactions and corporate finance\n".as_bytes()).expect("write failed");
        new_file.write_all(Transactions_and_corporate_finance.as_bytes()).expect("write failed");
    }
    _ => println!("Invalid selection");


}

