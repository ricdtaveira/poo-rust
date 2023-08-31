//
// Exemplo de uma Classe com
// métodos Contributors, Assessores e Modificadores
//
use rusqlite::{Connection, Result};
use std::error::Error;

// Definição da classe Aluno
struct Aluno {
    id: i32,
    nome: String,
    matricula: String,
    data_nascimento: String,
}

impl Aluno {
    // Construtor
    fn new(id: i32, nome: String, matricula: String, data_nascimento: String) -> Aluno {
        Aluno {
            id,
            nome,
            matricula,
            data_nascimento,
        }
    }

    // Assessores
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_nome(&self) -> &str {
        &self.nome
    }

    fn get_matricula(&self) -> &str {
        &self.matricula
    }

    fn get_data_nascimento(&self) -> &str {
        &self.data_nascimento
    }

    // Modificadores
    fn set_nome(&mut self, nome: String) {
        self.nome = nome;
    }

    fn set_matricula(&mut self, matricula: String) {
        self.matricula = matricula;
    }

    fn set_data_nascimento(&mut self, data_nascimento: String) {
        self.data_nascimento = data_nascimento;
    }
}

// Camada de Persistência
struct AlunoDao {
    connection: Connection,
}

impl AlunoDao {
    fn new() -> Result<Self> {
        let connection = Connection::open("alunos.db")?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS alunos (
                id INTEGER PRIMARY KEY,
                nome TEXT NOT NULL,
                matricula TEXT NOT NULL,
                data_nascimento TEXT NOT NULL
            )",
            [],
        )?;
        Ok(AlunoDao { connection })
    }

    fn insert(&mut self, aluno: &Aluno) -> Result<()> {
        self.connection.execute(
            "INSERT INTO alunos (id, nome, matricula, data_nascimento)
            VALUES (?1, ?2, ?3, ?4)",
            params![aluno.id, aluno.nome, aluno.matricula, aluno.data_nascimento],
        )?;
        Ok(())
    }

    fn update(&mut self, aluno: &Aluno) -> Result<()> {
        self.connection.execute(
            "UPDATE alunos SET nome = ?2, matricula = ?3, data_nascimento = ?4 WHERE id = ?1",
            params![aluno.id, aluno.nome, aluno.matricula, aluno.data_nascimento],
        )?;
        Ok(())
    }

    fn delete(&mut self, id: i32) -> Result<()> {
        self.connection.execute("DELETE FROM alunos WHERE id = ?1", params![id])?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut aluno_dao = AlunoDao::new()?;

    // Criando um aluno
    let aluno = Aluno::new(1, "João".to_string(), "12345".to_string(), "2000-01-01".to_string());

    // Inserindo o aluno no banco de dados
    aluno_dao.insert(&aluno)?;

    // Modificando o nome do aluno e atualizando no banco de dados
    let mut aluno_modificado = aluno.clone();
    aluno_modificado.set_nome("Pedro".to_string());
    aluno_dao.update(&aluno_modificado)?;

    // Deletando o aluno do banco de dados
    aluno_dao.delete(aluno.get_id())?;

    Ok(())
}
