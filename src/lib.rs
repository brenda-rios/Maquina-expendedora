use anchor_lang::prelude::*;

// El ID se llena automáticamente al hacer "build"
declare_id!("J2Eoyqqfjj9ti9SiTpmCwBqr28VwVgBgyEao6Hi14DzG"); 

#[program]
pub mod maquina_expendedora {
    use super::*;

    //Configurar o Crear la Máquina
    pub fn inicializar_maquina(context: Context<NuevaMaquina>, nombre_maquina: String) -> Result<()> {
        let owner = context.accounts.owner.key();
        let productos: Vec<Snack> = Vec::new(); 

        context.accounts.maquina.set_inner(Maquina {
            owner,
            nombre_maquina,
            inventario: productos,
        });
        msg!("Máquina '{}' creada exitosamente", context.accounts.maquina.nombre_maquina);
        Ok(())
    }

    //Agregar Snack
    pub fn agregar_snack(context: Context<ManejarSnack>, nombre: String, precio: u64, stock: u8) -> Result<()> {
        let maquina = &mut context.accounts.maquina;
        
        //Solo el dueño puede agregar productos
        require!(maquina.owner == context.accounts.owner.key(), Errores::NoAutorizado);

        msg!("Agregando el snack: {}", nombre);
        
        let nuevo_snack = Snack {
            nombre,
            precio,
            stock,
        };

        maquina.inventario.push(nuevo_snack);
        msg!("Snack agregado exitosamente.");
        Ok(())
    }

    //Ver Snacks (productos)
    pub fn ver_snacks(ctx: Context<ConsultarMaquina>) -> Result<()> {
        let maquina = &ctx.accounts.maquina;
        
        msg!("--- Inventario de la Máquina: {} ---", maquina.nombre_maquina);
        if maquina.inventario.is_empty() {
            msg!("La máquina está vacía actualmente.");
        } else {
            for snack in &maquina.inventario {
                msg!("Producto: {}, Precio: {}, Stock: {}", 
                    snack.nombre, 
                    snack.precio, 
                    snack.stock
                );
            }
        }
        
        Ok(())
    }

    //Vender Snacks - Resta stock
    pub fn vender_snack(context: Context<ManejarSnack>, nombre: String) -> Result<()> {
        let maquina = &mut context.accounts.maquina;
        let inventario = &mut maquina.inventario;

        for snack in inventario.iter_mut() {
            if snack.nombre == nombre {
                require!(snack.stock > 0, Errores::SinStock);
                snack.stock -= 1; // Reducimos el inventario en 1
                msg!("Venta realizada: {}. Stock restante: {}", nombre, snack.stock);
                return Ok(());
            }
        }
        Err(Errores::SnackNoEncontrado.into())
    }

    //Eliminar Productos
    pub fn retirar_snack(context: Context<ManejarSnack>, nombre: String) -> Result<()> {
        let maquina = &mut context.accounts.maquina;
        let inventario = &mut maquina.inventario;

        let posicion = inventario.iter().position(|x| x.nombre == nombre);
        
        if let Some(index) = posicion {
            inventario.remove(index);
            msg!("Producto {} retirado de la máquina.", nombre);
            return Ok(());
        }
        
        Err(Errores::SnackNoEncontrado.into())
    }
}

//Estructuras de Datos

#[account]
#[derive(InitSpace)]
pub struct Maquina {
    pub owner: Pubkey,
    #[max_len(60)]
    pub nombre_maquina: String,
    #[max_len(20)]
    pub inventario: Vec<Snack>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Snack {
    #[max_len(30)]
    pub nombre: String,
    pub precio: u64,
    pub stock: u8,
}

//Contextos de Cuentas

#[derive(Accounts)]
pub struct NuevaMaquina<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + Maquina::INIT_SPACE,
        seeds = [b"expendedora", owner.key().as_ref()],
        bump
    )]
    pub maquina: Account<'info, Maquina>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ManejarSnack<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub maquina: Account<'info, Maquina>,
}

#[derive(Accounts)]
pub struct ConsultarMaquina<'info> {
    pub maquina: Account<'info, Maquina>,
}

//Errores

#[error_code]
pub enum Errores {
    #[msg("No tienes permiso para modificar esta máquina.")]
    NoAutorizado,
    #[msg("El snack solicitado no existe en el inventario.")]
    SnackNoEncontrado,
    #[msg("Lo sentimos, este producto se ha agotado.")]
    SinStock,
}
