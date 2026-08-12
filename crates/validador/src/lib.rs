pub mod validadores {
    pub fn cpf(cpf: &str) -> bool {
        // Remove pontos e hífen
        let cpf: String = cpf.chars().filter(|c| c.is_ascii_digit()).collect();

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

    pub fn cnpj(cnpj: &str) -> bool {
        let cnpj: String = cnpj.chars().filter(|c| c.is_ascii_digit()).collect();

        // CNPJ deve possuir exatamente 14 dígitos
        if cnpj.len() != 14 {
            return false;
        }

        // Converte os caracteres para números
        let numeros: Vec<u32> = match cnpj
            .chars()
            .map(|c| c.to_digit(10))
            .collect::<Option<Vec<u32>>>()
        {
            Some(numeros) => numeros,
            None => return false,
        };

        // Rejeita CNPJs com todos os dígitos iguais
        if numeros.iter().all(|&numero| numero == numeros[0]) {
            return false;
        }

        // Primeiro dígito verificador
        let pesos1 = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

        let soma1: u32 = numeros[..12]
            .iter()
            .zip(pesos1.iter())
            .map(|(&numero, &peso)| numero * peso)
            .sum();

        let resto1 = soma1 % 11;
        let primeiro_digito = if resto1 < 2 { 0 } else { 11 - resto1 };

        if primeiro_digito != numeros[12] {
            return false;
        }

        // Segundo dígito verificador
        let pesos2 = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

        let soma2: u32 = numeros[..13]
            .iter()
            .zip(pesos2.iter())
            .map(|(&numero, &peso)| numero * peso)
            .sum();

        let resto2 = soma2 % 11;
        let segundo_digito = if resto2 < 2 { 0 } else { 11 - resto2 };

        segundo_digito == numeros[13]
    }
}
