#[derive(Debug)]
enum States {
    Q0,
    QC,
    QD,
    QDC,
    QCC,
    QCCC,
    QX,
    QL,
    QLX,
    QXX,
    QXXX,
    QI,
    QV,
    QVI,
    QII,
    QIII,
    Qtrap,
}


pub fn check_input(input:&String) -> bool{
    let allowed = vec!['M','D','C','L','X','V','I'];
    let mut state = States::Q0;

    for c in input.chars(){
        if ! allowed.contains(&c){
            println!("{} is not a romal numberal!", c);
            return false;
        }
        state = delta(&state, c);
    }
    match state {
        States::Qtrap => false,
        _ => true,
    }
}

fn delta(state:&States, c:char) -> States{
    match state {
        States::Q0 => match c {
            'M' => States::Q0,
            'D' => States::QD,
            'C' => States::QC,
            'L' => States::QL,
            'X' => States::QX,
            'V' => States::QV,
            'I' => States::QI,
            _ => States::Qtrap,
        },
        States::QC => match c {
            'M' | 'D' => States::QCCC,
            'C' => States::QCC,
            'L' => States::QL,
            'X' => States::QX,
            'V' => States::QV,
            'I' => States::QI,
            _ => States::Qtrap,
        },
        States::QD => match c {
            'C' => States::QDC,
            'L' => States::QL,
            'X' => States::QX,
            'V' => States::QV,
            'I' => States::QI,
            _ => States::Qtrap,
        },
        States::QDC => match c {
            'C' => States::QCC,
            'L' => States::QL,
            'X' => States::QX,
            'V' => States::QV,
            'I' => States::QI,
            _ => States::Qtrap,
        },
        States::QCC => match c {
            'C' => States::QCCC,
            'L' => States::QL,
            'X' => States::QX,
            'V' => States::QV,
            'I' => States::QI,
            _ => States::Qtrap,
        },
        States::QCCC => match c {
            'L' => States::QL,
            'X' => States::QX,
            'V' => States::QV,
            'I' => States::QI,
            _ => States::Qtrap,
        },
        States::QX => match c {
            'C' | 'L'=> States::QXXX,
            'X' => States::QXX,
            'V' => States::QV,
            'I' => States::QI,
            _ => States::Qtrap,
        },
        States::QL => match c {
            'X' => States::QLX,
            'V' => States::QV,
            'I' => States::QI,
            _ => States::Qtrap,
        },
        States::QLX => match c {
            'X' => States::QXX,
            'V' => States::QV,
            'I' => States::QI,
            _ => States::Qtrap,
        },
        States::QXX => match c {
            'X' => States::QXXX,
            'V' => States::QV,
            'I' => States::QI,
            _ => States::Qtrap,
        },
        States::QXXX => match c {
            'V' => States::QV,
            'I' => States::QI,
            _ => States::Qtrap,
        },
        States::QI => match c {
            'X' | 'V'=> States::QIII,
            'I' => States::QII,
            _ => States::Qtrap,
        },
        States::QV => match c {
            'I' => States::QVI,
            _ => States::Qtrap,
        },
        States::QVI => match c {
            'I' => States::QII,
            _ => States::Qtrap,
        },
        States::QII => match c {
            'I' => States::QIII,
            _ => States::Qtrap,
        },
        States::QIII | States::Qtrap => States::Qtrap,
    }
}
