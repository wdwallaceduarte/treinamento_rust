pub fn validar_cpf(cpf: &str) -> bool {
    // Remove pontos e hífen
    let cpf: String = cpf
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();

    // CPF precisa ter exatamente 11 dígitos
    if cpf.len() != 11 {
        return false;
    }

    // Converte os caracteres para números
    let numeros: Vec<u32> = match cpf
        .chars()
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<u32>>>()
    {
        Some(numeros) => numeros,
        None => return false,
    };

    // Rejeita CPFs com todos os dígitos iguais
    if numeros.iter().all(|&numero| numero == numeros[0]) {
        return false;
    }

    // Primeiro dígito verificador
    let mut soma = 0;

    for i in 0..9 {
        soma += numeros[i] * (10 - i as u32);
    }

    let resto = (soma * 10) % 11;
    let primeiro_digito = if resto == 10 { 0 } else { resto };

    if primeiro_digito != numeros[9] {
        return false;
    }

    // Segundo dígito verificador
    let mut soma = 0;

    for i in 0..10 {
        soma += numeros[i] * (11 - i as u32);
    }

    let resto = (soma * 10) % 11;
    let segundo_digito = if resto == 10 { 0 } else { resto };

    segundo_digito == numeros[10]
}


