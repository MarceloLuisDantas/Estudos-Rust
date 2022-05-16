pub fn sintaxe_var_const(parametros: &[&str]) -> Result<bool, String> {
    if parametros.len() != 3 {
        Err("A função 'Var'/'Const' recebem exatamente 2 parametros \n 
             -- var [nome] [valor] \n
             -- const [nome] [valor] \n".trim().to_string())
    } else {
        match parametros[1].parse::<f64>() {
            Ok(_) => Err(format!("{} não é um nome valido", parametros[1])),
            Err(_) => Ok(true),
        }
    }
}

pub fn sintaxe_set(parametros: &[&str]) -> Result <bool, String> {
    if parametros.len() != 3 {
        Err("A função 'Set' recebem exatamente 2 parametros \n 
             -- set [valor] [nome] \n".trim().to_string())
    } else {
        Ok(true)
    }
}

pub fn sintaxe_type_of(parametros: &[&str]) -> Result <bool, String> {
    if parametros.len() != 2 {
        Err("A função 'type_of' recebem exatamente 1 parametro \n 
             -- type_of [nome] \n".trim().to_string())
    } else {
        Ok(true)
    }
}

pub fn sintaxe_rmv(parametros: &[&str]) -> Result<bool, String> {
    if parametros.len() != 2 {
        Err("A função 'Rmv' recebe exatamente 1 parmetro \n
             -- rmv [nome] \n".trim().to_string())
    } else {
        Ok(true)
    }
}

pub fn sintaxe_aritmetica(parametros: &[&str]) -> Result<bool, String> {
    let operadores: Vec<f32> = parametros
        .iter()
        .filter_map(|x| x.parse().ok())
        .collect();

    if operadores.len() != parametros.len() - 1 {
        Err("Funções de aritmetica recebem apenas numeros \n
             -- sum 40 23 \n
             -- sub 21 4 \n
             -- div 4.12 2.0 \n
             -- mult 3 5 \n".to_string())
    } else {
        Ok(true)
    }
}