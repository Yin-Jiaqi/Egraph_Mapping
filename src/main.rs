mod lib;
mod dynamic_rules;
mod static_rules;
mod lp;
use egg::*;
use egraph_mapping::{SimpleLanguage, convert_to_simple_language_enum, get_node_type};
use std::collections::HashMap;
use std::time::Instant;
use std::time::Duration;



// #[derive(Debug)]
// pub struct CombinedCost;

// impl CostFunction<SimpleLanguage> for CombinedCost {
//     type Cost = (f64, f64, f64);

//     fn cost<C>(&mut self, enode: &SimpleLanguage, mut cost: C) -> Self::Cost
//     where
//         C: FnMut(Id) -> Self::Cost,
//     {
//         let cells = lib::parse_genlib("7nm.genlib");

//         // Helper function to get the node type as a string

//         let (node_size, node_depth) = match enode {
//             SimpleLanguage::Symbol(_) => (0.0, 0.0),
//             _ => {
//                 let node_type = get_node_type(enode);
//                 cells.iter()
//                     .find(|&&(ref name, _, _, _)| name == node_type)
//                     .map_or((5000.0, 5000.0), |&(_, size, depth, _)| (size, depth))
//             }
//         };

//         let ast_size = node_size + enode.fold(0.0_f64, |sum, id| sum + cost(id).1);
//         let ast_depth = node_depth + enode.fold(0.0_f64, |max, id| max.max(cost(id).2));
//         (ast_size, ast_size, ast_depth)
//     }
// }

pub struct WeightedAstSize;
impl CostFunction<SimpleLanguage> for WeightedAstSize {
    type Cost = f64;
    fn cost<C>(&mut self, enode: &SimpleLanguage, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        let cells = [("_const0_", 0.0, 1.0, "z=CONST0"), ("_const1_", 0.0, 1.0, "z=CONST1"), ("BUFx10_ASAP7_75t_L", 3.27, 1.0, "Y=A"), ("BUFx12_ASAP7_75t_L", 3.73, 1.0, "Y=A"), ("BUFx12f_ASAP7_75t_L", 4.2, 1.0, "Y=A"), ("BUFx16f_ASAP7_75t_L", 5.13, 1.0, "Y=A"), ("BUFx24_ASAP7_75t_L", 7.0, 1.0, "Y=A"), ("BUFx2_ASAP7_75t_L", 1.17, 1.0, "Y=A"), ("BUFx3_ASAP7_75t_L", 1.4, 1.0, "Y=A"), ("BUFx4_ASAP7_75t_L", 1.63, 1.0, "Y=A"), ("BUFx4f_ASAP7_75t_L", 1.87, 1.0, "Y=A"), ("BUFx5_ASAP7_75t_L", 1.87, 1.0, "Y=A"), ("BUFx6f_ASAP7_75t_L", 2.33, 1.0, "Y=A"), ("BUFx8_ASAP7_75t_L", 2.8, 1.0, "Y=A"), ("HB1xp67_ASAP7_75t_L", 0.93, 1.0, "Y=A"), ("HB2xp67_ASAP7_75t_L", 1.17, 1.0, "Y=A"), ("HB3xp67_ASAP7_75t_L", 1.4, 1.0, "Y=A"), ("HB4xp67_ASAP7_75t_L", 1.63, 1.0, "Y=A"), ("INVx11_ASAP7_75t_L", 3.03, 1.0, "Y=!A"), ("INVx13_ASAP7_75t_L", 3.5, 1.0, "Y=!A"), ("INVx1_ASAP7_75t_L", 0.7, 1.0, "Y=!A"), ("INVx2_ASAP7_75t_L", 0.93, 1.0, "Y=!A"), ("INVx3_ASAP7_75t_L", 1.17, 1.0, "Y=!A"), ("INVx4_ASAP7_75t_L", 1.4, 1.0, "Y=!A"), ("INVx5_ASAP7_75t_L", 1.63, 1.0, "Y=!A"), ("INVx6_ASAP7_75t_L", 1.87, 1.0, "Y=!A"), ("INVx8_ASAP7_75t_L", 2.33, 1.0, "Y=!A"), ("INVxp33_ASAP7_75t_L", 0.7, 1.0, "Y=!A"), ("INVxp67_ASAP7_75t_L", 0.7, 1.0, "Y=!A"), ("AND2x2_ASAP7_75t_L", 1.4, 1.0, "Y=(A * B)"), ("AND2x4_ASAP7_75t_L", 2.33, 1.0, "Y=(A * B)"), ("AND2x6_ASAP7_75t_L", 2.8, 1.0, "Y=(A * B)"), ("AND3x1_ASAP7_75t_L", 1.4, 1.0, "Y=(A * B * C)"), ("AND3x2_ASAP7_75t_L", 1.63, 1.0, "Y=(A * B * C)"), ("AND3x4_ASAP7_75t_L", 3.73, 1.0, "Y=(A * B * C)"), ("AND4x1_ASAP7_75t_L", 1.63, 1.0, "Y=(A * B * C * D)"), ("AND4x2_ASAP7_75t_L", 3.73, 1.0, "Y=(A * B * C * D)"), ("AND5x1_ASAP7_75t_L", 1.87, 1.0, "Y=(A * B * C * D * E)"), ("AND5x2_ASAP7_75t_L", 4.67, 1.0, "Y=(A * B * C * D * E)"), ("FAx1_ASAP7_75t_L", 3.27, 1.0, "CON=(!A * !B) + (!A * !CI) + (!B * !CI)"), ("FAx1_ASAP7_75t_L", 3.27, 1.0, "SN=(A * B * !CI) + (A * !B * CI) + (!A * B * CI) + (!A * !B * !CI)"), ("HAxp5_ASAP7_75t_L", 2.1, 1.0, "CON=(!A) + (!B)"), ("HAxp5_ASAP7_75t_L", 2.1, 1.0, "SN=(A * B) + (!A * !B)"), ("MAJIxp5_ASAP7_75t_L", 1.63, 1.0, "Y=(!A * !B) + (!A * !C) + (!B * !C)"), ("MAJx2_ASAP7_75t_L", 2.1, 1.0, "Y=(A * B) + (A * C) + (B * C)"), ("MAJx3_ASAP7_75t_L", 2.33, 1.0, "Y=(A * B) + (A * C) + (B * C)"), ("NAND2x1_ASAP7_75t_L", 1.4, 1.0, "Y=(!A) + (!B)"), ("NAND2x1p5_ASAP7_75t_L", 1.87, 1.0, "Y=(!A) + (!B)"), ("NAND2x2_ASAP7_75t_L", 2.33, 1.0, "Y=(!A) + (!B)"), ("NAND2xp33_ASAP7_75t_L", 0.93, 1.0, "Y=(!A) + (!B)"), ("NAND2xp5_ASAP7_75t_L", 0.93, 1.0, "Y=(!A) + (!B)"), ("NAND2xp67_ASAP7_75t_L", 1.4, 1.0, "Y=(!A) + (!B)"), ("NAND3x1_ASAP7_75t_L", 2.57, 1.0, "Y=(!A) + (!B) + (!C)"), ("NAND3x2_ASAP7_75t_L", 4.67, 1.0, "Y=(!A) + (!B) + (!C)"), ("NAND3xp33_ASAP7_75t_L", 1.17, 1.0, "Y=(!A) + (!B) + (!C)"), ("NAND4xp25_ASAP7_75t_L", 1.4, 1.0, "Y=(!A) + (!B) + (!C) + (!D)"), ("NAND4xp75_ASAP7_75t_L", 3.27, 1.0, "Y=(!A) + (!B) + (!C) + (!D)"), ("NAND5xp2_ASAP7_75t_L", 1.63, 1.0, "Y=(!A) + (!B) + (!C) + (!D) + (!E)"), ("NOR2x1_ASAP7_75t_L", 1.4, 1.0, "Y=(!A * !B)"), ("NOR2x1p5_ASAP7_75t_L", 1.87, 1.0, "Y=(!A * !B)"), ("NOR2x2_ASAP7_75t_L", 2.33, 1.0, "Y=(!A * !B)"), ("NOR2xp33_ASAP7_75t_L", 0.93, 1.0, "Y=(!A * !B)"), ("NOR2xp67_ASAP7_75t_L", 1.4, 1.0, "Y=(!A * !B)"), ("NOR3x1_ASAP7_75t_L", 2.57, 1.0, "Y=(!A * !B * !C)"), ("NOR3x2_ASAP7_75t_L", 4.67, 1.0, "Y=(!A * !B * !C)"), ("NOR3xp33_ASAP7_75t_L", 1.17, 1.0, "Y=(!A * !B * !C)"), ("NOR4xp25_ASAP7_75t_L", 1.4, 1.0, "Y=(!A * !B * !C * !D)"), ("NOR4xp75_ASAP7_75t_L", 3.27, 1.0, "Y=(!A * !B * !C * !D)"), ("NOR5xp2_ASAP7_75t_L", 1.63, 1.0, "Y=(!A * !B * !C * !D * !E)"), ("OR2x2_ASAP7_75t_L", 1.4, 1.0, "Y=(A) + (B)"), ("OR2x4_ASAP7_75t_L", 1.87, 1.0, "Y=(A) + (B)"), ("OR2x6_ASAP7_75t_L", 2.8, 1.0, "Y=(A) + (B)"), ("OR3x1_ASAP7_75t_L", 1.4, 1.0, "Y=(A) + (B) + (C)"), ("OR3x2_ASAP7_75t_L", 1.63, 1.0, "Y=(A) + (B) + (C)"), ("OR3x4_ASAP7_75t_L", 2.1, 1.0, "Y=(A) + (B) + (C)"), ("OR4x1_ASAP7_75t_L", 1.63, 1.0, "Y=(A) + (B) + (C) + (D)"), ("OR4x2_ASAP7_75t_L", 1.87, 1.0, "Y=(A) + (B) + (C) + (D)"), ("OR5x1_ASAP7_75t_L", 1.87, 1.0, "Y=(A) + (B) + (C) + (D) + (E)"), ("OR5x2_ASAP7_75t_L", 2.1, 1.0, "Y=(A) + (B) + (C) + (D) + (E)"), ("XNOR2x1_ASAP7_75t_L", 2.8, 1.0, "Y=(A * B) + (!A * !B)"), ("XNOR2x2_ASAP7_75t_L", 2.57, 1.0, "Y=(A * B) + (!A * !B)"), ("XNOR2xp5_ASAP7_75t_L", 2.1, 1.0, "Y=(A * B) + (!A * !B)"), ("XOR2x1_ASAP7_75t_L", 2.8, 1.0, "Y=(A * !B) + (!A * B)"), ("XOR2x2_ASAP7_75t_L", 2.57, 1.0, "Y=(A * !B) + (!A * B)"), ("XOR2xp5_ASAP7_75t_L", 2.1, 1.0, "Y=(A * !B) + (!A * B)"), ("A2O1A1Ixp33_ASAP7_75t_L", 0.0, 1.0, "Y=(!A1 * !B) + (!A2 * !B) + (!C)"), ("A2O1A1O1Ixp25_ASAP7_75t_L", 0.0, 1.0, "Y=(!A1 * !B * !D) + (!A2 * !B * !D) + (!C * !D)"), ("AO211x2_ASAP7_75t_L", 3.73, 1.0, "Y=(A1 * A2) + (B) + (C)"), ("AO21x1_ASAP7_75t_L", 1.4, 1.0, "Y=(A1 * A2) + (B)"), ("AO21x2_ASAP7_75t_L", 1.63, 1.0, "Y=(A1 * A2) + (B)"), ("AO221x1_ASAP7_75t_L", 2.33, 1.0, "Y=(A1 * A2) + (B1 * B2) + (C)"), ("AO221x2_ASAP7_75t_L", 2.57, 1.0, "Y=(A1 * A2) + (B1 * B2) + (C)"), ("AO222x2_ASAP7_75t_L", 5.13, 1.0, "Y=(A1 * A2) + (B1 * B2) + (C1 * C2)"), ("AO22x1_ASAP7_75t_L", 2.1, 1.0, "Y=(A1 * A2) + (B1 * B2)"), ("AO22x2_ASAP7_75t_L", 2.33, 1.0, "Y=(A1 * A2) + (B1 * B2)"), ("AO31x2_ASAP7_75t_L", 3.73, 1.0, "Y=(A1 * A2 * A3) + (B)"), ("AO322x2_ASAP7_75t_L", 3.5, 1.0, "Y=(A1 * A2 * A3) + (B1 * B2) + (C1 * C2)"), ("AO32x1_ASAP7_75t_L", 1.87, 1.0, "Y=(A1 * A2 * A3) + (B1 * B2)"), ("AO32x2_ASAP7_75t_L", 2.1, 1.0, "Y=(A1 * A2 * A3) + (B1 * B2)"), ("AO331x1_ASAP7_75t_L", 2.33, 1.0, "Y=(A1 * A2 * A3) + (B1 * B2 * B3) + (C)"), ("AO331x2_ASAP7_75t_L", 2.57, 1.0, "Y=(A1 * A2 * A3) + (B1 * B2 * B3) + (C)"), ("AO332x1_ASAP7_75t_L", 2.57, 1.0, "Y=(A1 * A2 * A3) + (B1 * B2 * B3) + (C1 * C2)"), ("AO332x2_ASAP7_75t_L", 2.8, 1.0, "Y=(A1 * A2 * A3) + (B1 * B2 * B3) + (C1 * C2)"), ("AO333x1_ASAP7_75t_L", 2.8, 1.0, "Y=(A1 * A2 * A3) + (B1 * B2 * B3) + (C1 * C2 * C3)"), ("AO333x2_ASAP7_75t_L", 3.03, 1.0, "Y=(A1 * A2 * A3) + (B1 * B2 * B3) + (C1 * C2 * C3)"), ("AO33x2_ASAP7_75t_L", 2.33, 1.0, "Y=(A1 * A2 * A3) + (B1 * B2 * B3)"), ("AOI211x1_ASAP7_75t_L", 2.8, 1.0, "Y=(!A1 * !B * !C) + (!A2 * !B * !C)"), ("AOI211xp5_ASAP7_75t_L", 1.4, 1.0, "Y=(!A1 * !B * !C) + (!A2 * !B * !C)"), ("AOI21x1_ASAP7_75t_L", 1.87, 1.0, "Y=(!A1 * !B) + (!A2 * !B)"), ("AOI21xp33_ASAP7_75t_L", 1.17, 1.0, "Y=(!A1 * !B) + (!A2 * !B)"), ("AOI21xp5_ASAP7_75t_L", 1.17, 1.0, "Y=(!A1 * !B) + (!A2 * !B)"), ("AOI221x1_ASAP7_75t_L", 3.27, 1.0, "Y=(!A1 * !B1 * !C) + (!A1 * !B2 * !C) + (!A2 * !B1 * !C) + (!A2 * !B2 * !C)"), ("AOI221xp5_ASAP7_75t_L", 1.63, 1.0, "Y=(!A1 * !B1 * !C) + (!A1 * !B2 * !C) + (!A2 * !B1 * !C) + (!A2 * !B2 * !C)"), ("AOI222xp33_ASAP7_75t_L", 2.33, 1.0, "Y=(!A1 * !B1 * !C1) + (!A1 * !B1 * !C2) + (!A1 * !B2 * !C1) + (!A1 * !B2 * !C2) + (!A2 * !B1 * !C1) + (!A2 * !B1 * !C2) + (!A2 * !B2 * !C1) + (!A2 * !B2 * !C2)"), ("AOI22x1_ASAP7_75t_L", 2.33, 1.0, "Y=(!A1 * !B1) + (!A1 * !B2) + (!A2 * !B1) + (!A2 * !B2)"), ("AOI22xp33_ASAP7_75t_L", 1.4, 1.0, "Y=(!A1 * !B1) + (!A1 * !B2) + (!A2 * !B1) + (!A2 * !B2)"), ("AOI22xp5_ASAP7_75t_L", 1.4, 1.0, "Y=(!A1 * !B1) + (!A1 * !B2) + (!A2 * !B1) + (!A2 * !B2)"), ("AOI311xp33_ASAP7_75t_L", 1.63, 1.0, "Y=(!A1 * !B * !C) + (!A2 * !B * !C) + (!A3 * !B * !C)"), ("AOI31xp33_ASAP7_75t_L", 1.4, 1.0, "Y=(!A1 * !B) + (!A2 * !B) + (!A3 * !B)"), ("AOI31xp67_ASAP7_75t_L", 3.03, 1.0, "Y=(!A1 * !B) + (!A2 * !B) + (!A3 * !B)"), ("AOI321xp33_ASAP7_75t_L", 1.87, 1.0, "Y=(!A1 * !B1 * !C) + (!A1 * !B2 * !C) + (!A2 * !B1 * !C) + (!A2 * !B2 * !C) + (!A3 * !B1 * !C) + (!A3 * !B2 * !C)"), ("AOI322xp5_ASAP7_75t_L", 2.1, 1.0, "Y=(!A1 * !B1 * !C1) + (!A1 * !B1 * !C2) + (!A1 * !B2 * !C1) + (!A1 * !B2 * !C2) + (!A2 * !B1 * !C1) + (!A2 * !B1 * !C2) + (!A2 * !B2 * !C1) + (!A2 * !B2 * !C2) + (!A3 * !B1 * !C1) + (!A3 * !B1 * !C2) + (!A3 * !B2 * !C1) + (!A3 * !B2 * !C2)"), ("AOI32xp33_ASAP7_75t_L", 1.63, 1.0, "Y=(!A1 * !B1) + (!A1 * !B2) + (!A2 * !B1) + (!A2 * !B2) + (!A3 * !B1) + (!A3 * !B2)"), ("AOI331xp33_ASAP7_75t_L", 2.1, 1.0, "Y=(!A1 * !B1 * !C1) + (!A1 * !B2 * !C1) + (!A1 * !B3 * !C1) + (!A2 * !B1 * !C1) + (!A2 * !B2 * !C1) + (!A2 * !B3 * !C1) + (!A3 * !B1 * !C1) + (!A3 * !B2 * !C1) + (!A3 * !B3 * !C1)"), ("AOI332xp33_ASAP7_75t_L", 2.33, 1.0, "Y=(!A1 * !B1 * !C1) + (!A1 * !B1 * !C2) + (!A1 * !B2 * !C1) + (!A1 * !B2 * !C2) + (!A1 * !B3 * !C1) + (!A1 * !B3 * !C2) + (!A2 * !B1 * !C1) + (!A2 * !B1 * !C2) + (!A2 * !B2 * !C1) + (!A2 * !B2 * !C2) + (!A2 * !B3 * !C1) + (!A2 * !B3 * !C2) + (!A3 * !B1 * !C1) + (!A3 * !B1 * !C2) + (!A3 * !B2 * !C1) + (!A3 * !B2 * !C2) + (!A3 * !B3 * !C1) + (!A3 * !B3 * !C2)"), ("AOI333xp33_ASAP7_75t_L", 2.57, 1.0, "Y=(!A1 * !B1 * !C1) + (!A1 * !B1 * !C2) + (!A1 * !B1 * !C3) + (!A1 * !B2 * !C1) + (!A1 * !B2 * !C2) + (!A1 * !B2 * !C3) + (!A1 * !B3 * !C1) + (!A1 * !B3 * !C2) + (!A1 * !B3 * !C3) + (!A2 * !B1 * !C1) + (!A2 * !B1 * !C2) + (!A2 * !B1 * !C3) + (!A2 * !B2 * !C1) + (!A2 * !B2 * !C2) + (!A2 * !B2 * !C3) + (!A2 * !B3 * !C1) + (!A2 * !B3 * !C2) + (!A2 * !B3 * !C3) + (!A3 * !B1 * !C1) + (!A3 * !B1 * !C2) + (!A3 * !B1 * !C3) + (!A3 * !B2 * !C1) + (!A3 * !B2 * !C2) + (!A3 * !B2 * !C3) + (!A3 * !B3 * !C1) + (!A3 * !B3 * !C2) + (!A3 * !B3 * !C3)"), ("AOI33xp33_ASAP7_75t_L", 1.87, 1.0, "Y=(!A1 * !B1) + (!A1 * !B2) + (!A1 * !B3) + (!A2 * !B1) + (!A2 * !B2) + (!A2 * !B3) + (!A3 * !B1) + (!A3 * !B2) + (!A3 * !B3)"), ("O2A1O1Ixp33_ASAP7_75t_L", 0.0, 1.0, "Y=(!A1 * !A2 * !C) + (!B * !C)"), ("O2A1O1Ixp5_ASAP7_75t_L", 0.0, 1.0, "Y=(!A1 * !A2 * !C) + (!B * !C)"), ("OA211x2_ASAP7_75t_L", 1.87, 1.0, "Y=(A1 * B * C) + (A2 * B * C)"), ("OA21x2_ASAP7_75t_L", 1.63, 1.0, "Y=(A1 * B) + (A2 * B)"), ("OA221x2_ASAP7_75t_L", 3.73, 1.0, "Y=(A1 * B1 * C) + (A1 * B2 * C) + (A2 * B1 * C) + (A2 * B2 * C)"), ("OA222x2_ASAP7_75t_L", 2.8, 1.0, "Y=(A1 * B1 * C1) + (A1 * B1 * C2) + (A1 * B2 * C1) + (A1 * B2 * C2) + (A2 * B1 * C1) + (A2 * B1 * C2) + (A2 * B2 * C1) + (A2 * B2 * C2)"), ("OA22x2_ASAP7_75t_L", 2.33, 1.0, "Y=(A1 * B1) + (A1 * B2) + (A2 * B1) + (A2 * B2)"), ("OA31x2_ASAP7_75t_L", 3.5, 1.0, "Y=(A1 * B1) + (A2 * B1) + (A3 * B1)"), ("OA331x1_ASAP7_75t_L", 2.33, 1.0, "Y=(A1 * B1 * C1) + (A1 * B2 * C1) + (A1 * B3 * C1) + (A2 * B1 * C1) + (A2 * B2 * C1) + (A2 * B3 * C1) + (A3 * B1 * C1) + (A3 * B2 * C1) + (A3 * B3 * C1)"), ("OA331x2_ASAP7_75t_L", 2.57, 1.0, "Y=(A1 * B1 * C1) + (A1 * B2 * C1) + (A1 * B3 * C1) + (A2 * B1 * C1) + (A2 * B2 * C1) + (A2 * B3 * C1) + (A3 * B1 * C1) + (A3 * B2 * C1) + (A3 * B3 * C1)"), ("OA332x1_ASAP7_75t_L", 2.57, 1.0, "Y=(A1 * B1 * C1) + (A1 * B1 * C2) + (A1 * B2 * C1) + (A1 * B2 * C2) + (A1 * B3 * C1) + (A1 * B3 * C2) + (A2 * B1 * C1) + (A2 * B1 * C2) + (A2 * B2 * C1) + (A2 * B2 * C2) + (A2 * B3 * C1) + (A2 * B3 * C2) + (A3 * B1 * C1) + (A3 * B1 * C2) + (A3 * B2 * C1) + (A3 * B2 * C2) + (A3 * B3 * C1) + (A3 * B3 * C2)"), ("OA332x2_ASAP7_75t_L", 2.8, 1.0, "Y=(A1 * B1 * C1) + (A1 * B1 * C2) + (A1 * B2 * C1) + (A1 * B2 * C2) + (A1 * B3 * C1) + (A1 * B3 * C2) + (A2 * B1 * C1) + (A2 * B1 * C2) + (A2 * B2 * C1) + (A2 * B2 * C2) + (A2 * B3 * C1) + (A2 * B3 * C2) + (A3 * B1 * C1) + (A3 * B1 * C2) + (A3 * B2 * C1) + (A3 * B2 * C2) + (A3 * B3 * C1) + (A3 * B3 * C2)"), ("OA333x1_ASAP7_75t_L", 2.8, 1.0, "Y=(A1 * B1 * C1) + (A1 * B1 * C2) + (A1 * B1 * C3) + (A1 * B2 * C1) + (A1 * B2 * C2) + (A1 * B2 * C3) + (A1 * B3 * C1) + (A1 * B3 * C2) + (A1 * B3 * C3) + (A2 * B1 * C1) + (A2 * B1 * C2) + (A2 * B1 * C3) + (A2 * B2 * C1) + (A2 * B2 * C2) + (A2 * B2 * C3) + (A2 * B3 * C1) + (A2 * B3 * C2) + (A2 * B3 * C3) + (A3 * B1 * C1) + (A3 * B1 * C2) + (A3 * B1 * C3) + (A3 * B2 * C1) + (A3 * B2 * C2) + (A3 * B2 * C3) + (A3 * B3 * C1) + (A3 * B3 * C2) + (A3 * B3 * C3)"), ("OA333x2_ASAP7_75t_L", 3.03, 1.0, "Y=(A1 * B1 * C1) + (A1 * B1 * C2) + (A1 * B1 * C3) + (A1 * B2 * C1) + (A1 * B2 * C2) + (A1 * B2 * C3) + (A1 * B3 * C1) + (A1 * B3 * C2) + (A1 * B3 * C3) + (A2 * B1 * C1) + (A2 * B1 * C2) + (A2 * B1 * C3) + (A2 * B2 * C1) + (A2 * B2 * C2) + (A2 * B2 * C3) + (A2 * B3 * C1) + (A2 * B3 * C2) + (A2 * B3 * C3) + (A3 * B1 * C1) + (A3 * B1 * C2) + (A3 * B1 * C3) + (A3 * B2 * C1) + (A3 * B2 * C2) + (A3 * B2 * C3) + (A3 * B3 * C1) + (A3 * B3 * C2) + (A3 * B3 * C3)"), ("OA33x2_ASAP7_75t_L", 2.33, 1.0, "Y=(A1 * B1) + (A1 * B2) + (A1 * B3) + (A2 * B1) + (A2 * B2) + (A2 * B3) + (A3 * B1) + (A3 * B2) + (A3 * B3)"), ("OAI211xp5_ASAP7_75t_L", 1.4, 1.0, "Y=(!A1 * !A2) + (!B) + (!C)"), ("OAI21x1_ASAP7_75t_L", 1.87, 1.0, "Y=(!A1 * !A2) + (!B)"), ("OAI21xp33_ASAP7_75t_L", 1.17, 1.0, "Y=(!A1 * !A2) + (!B)"), ("OAI21xp5_ASAP7_75t_L", 1.17, 1.0, "Y=(!A1 * !A2) + (!B)"), ("OAI221xp5_ASAP7_75t_L", 1.63, 1.0, "Y=(!A1 * !A2) + (!B1 * !B2) + (!C)"), ("OAI222xp33_ASAP7_75t_L", 2.33, 1.0, "Y=(!A1 * !A2) + (!B1 * !B2) + (!C1 * !C2)"), ("OAI22x1_ASAP7_75t_L", 2.33, 1.0, "Y=(!A1 * !A2) + (!B1 * !B2)"), ("OAI22xp33_ASAP7_75t_L", 1.4, 1.0, "Y=(!A1 * !A2) + (!B1 * !B2)"), ("OAI22xp5_ASAP7_75t_L", 1.4, 1.0, "Y=(!A1 * !A2) + (!B1 * !B2)"), ("OAI311xp33_ASAP7_75t_L", 1.63, 1.0, "Y=(!A1 * !A2 * !A3) + (!B1) + (!C1)"), ("OAI31xp33_ASAP7_75t_L", 1.4, 1.0, "Y=(!A1 * !A2 * !A3) + (!B)"), ("OAI31xp67_ASAP7_75t_L", 3.03, 1.0, "Y=(!A1 * !A2 * !A3) + (!B)"), ("OAI321xp33_ASAP7_75t_L", 1.87, 1.0, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2) + (!C)"), ("OAI322xp33_ASAP7_75t_L", 2.1, 1.0, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2) + (!C1 * !C2)"), ("OAI32xp33_ASAP7_75t_L", 1.63, 1.0, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2)"), ("OAI331xp33_ASAP7_75t_L", 2.1, 1.0, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2 * !B3) + (!C1)"), ("OAI332xp33_ASAP7_75t_L", 2.33, 1.0, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2 * !B3) + (!C1 * !C2)"), ("OAI333xp33_ASAP7_75t_L", 2.57, 1.0, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2 * !B3) + (!C1 * !C2 * !C3)"), ("OAI33xp33_ASAP7_75t_L", 2.57, 1.0, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2 * !B3) + (!C1 * !C2 * !C3)")];
        let (node_size, _) = match enode {
            SimpleLanguage::Symbol(_) => (0.0, 0.0),
            SimpleLanguage::OUT(_) => (0.0, 0.0),
            _ => {
                let node_type = &get_node_type(enode);
                cells.iter()
                    .find(|&&(ref name, _, _, _)| name == node_type)
                    .map_or((5000.0, 5000.0), |&(_, size, depth, _)| (size, depth))
            }
        };
        enode.fold(node_size, |sum, id| sum + costs(id))
    }
}



fn simplify_programmatically(expr: &RecExpr<SimpleLanguage>) -> (std::time::Duration,std::time::Duration,std::time::Duration,f64, RecExpr<SimpleLanguage>, RecExpr<SimpleLanguage>,Runner<SimpleLanguage,()>){
    println!("Start E-graph Grownth");
    let start = Instant::now();
    let runner = Runner::default().with_iter_limit(10_000_000_000).with_node_limit(10_000_000_000).with_time_limit(Duration::from_secs(60)).with_expr(expr).run(&static_rules::make_rules());
    let grownth_duration = start.elapsed();
    let root = runner.roots[0];
    let start = Instant::now();
    println!("Start Greedy  Extraction");
    let extractor = Extractor::new(&runner.egraph, WeightedAstSize);
    let (best_cost, best) = extractor.find_best(root);
    let greedy_duration = start.elapsed();
    let cells = lib::parse_genlib("7nm.genlib");
    let start = Instant::now();
    println!("Start LP  Extraction");
    let lp_best = lp::LpExtractor::new(&runner.egraph, AstSize,cells).solve(root);
    let lp_duration = start.elapsed();

    (grownth_duration, greedy_duration, lp_duration, best_cost as f64, best, lp_best,runner)
}


fn main() {
    let benchmark = "c2670";
    let path = "benchmark/iscas85/";
    let blif_path = "benchmark/synthetic/c2670/";
    // match lib::parse_graph("benchmark/iscas85/c7552.bench") {
    match lib::parse_graph(&format!("{}{}.bench", path, benchmark)) {
        Ok((vertices, _, in_pairs, _ ,edge_pairs, input, output)) => {
            let mut expr: RecExpr<SimpleLanguage> = RecExpr::default();
            let mut vertices_map: HashMap<String, Id> = HashMap::new();
            let mut queue: Vec<String> = vertices.clone();
            let mut out_id: HashMap<String, Id> = HashMap::new();
            queue.reverse();
            while !queue.is_empty(){
                let operation=queue.pop().unwrap();
                if let Some(input_edges) = in_pairs.get(&operation) {
                    let mut in_id = Vec::new();
                    for vertex in edge_pairs.get(input_edges).unwrap_or(&Vec::new()){
                        if let Some(id) = vertices_map.get(vertex) {
                            in_id.push(*id);
                        }
                    }
                    let name: Vec<&str> = input_edges.split("_").collect();
                    if name.len()==2 {
                        let stripped_name = name.first().unwrap().to_lowercase();
                        if let Some(value) = convert_to_simple_language_enum (in_id, &stripped_name) {
                            let temp_id = expr.add(value);
                            if output.contains(&operation){
                                out_id.insert(operation.clone(), temp_id);
                            }
                            vertices_map.insert(operation.clone(), temp_id);
                            queue.retain(|x| x != &operation);
                        }
                        else {
                            panic!("Unknown enum variant for {}", stripped_name);
                        }
                    }
                    else {
                        panic!("Edge format error: {}", input_edges);
                    }
                } else {
                    let temp_id = expr.add(SimpleLanguage::Symbol(operation.clone().into()));
                    if output.contains(&operation){
                        out_id.insert(operation.clone(), temp_id);
                    }
                    vertices_map.insert(operation.clone(), temp_id);
                    queue.retain(|x| x != &operation);
                }
            }

            let mut vec_out_id: Vec<Id> = Vec::new();
            for element in &output{
                vec_out_id.push(out_id[element]);
            }

            let value = SimpleLanguage::OUT(vec_out_id);
            let temp_id = expr.add(value);
            vertices_map.insert("output".to_string(), temp_id);
            
            let (grownth_duration, greedy_duration, lp_duration, _, greedy_best_expr, lp_best_expr, runner) = simplify_programmatically(&expr);

            // let mut file = File::create("greedy_output.txt").expect("Failed to create file");
            // write!(file, "{}", greedy_best_expr).expect("Failed to write data to file");
            // let mut file = File::create("LP_output.txt").expect("Failed to create file");
            // write!(file, "{}", lp_best_expr).expect("Failed to write data to file");
            let mut greedy_counts: HashMap<&str, usize> = HashMap::new();
            for node in greedy_best_expr.as_ref() {
                let node_type = get_node_type(node);
                *greedy_counts.entry(node_type).or_insert(0) += 1;
            }
            // println!("{:?}",greedy_counts);
            let mut lp_counts: HashMap<&str, usize> = HashMap::new();
            for node in lp_best_expr.as_ref() {
                let node_type = get_node_type(node);
                *lp_counts.entry(node_type).or_insert(0) += 1;
            }

            println!("----------------------------------------------------------------------");
            println!("Runtime:");
            println!("      Saturation: {:?}",grownth_duration);
            println!("      Greedy    : {:?}",greedy_duration);
            println!("      ILP       : {:?}",lp_duration);

            println!("Expr:");
            println!("      Greedy: {:?}",greedy_best_expr);
            println!("      ILP   : {:?}",lp_best_expr);
            println!(" Original   : {:?}",expr);
            
            if let Err(e) = lib::parse2blif(&blif_path, &benchmark, &lp_best_expr,input,output) {
                eprintln!("Failed to parse to BLIF: {}", e);
            }

            println!("Counts:");
            println!("      Greedy: {:?}",greedy_counts);
            println!("      Count : {:?}",lp_counts);
            println!("----------------------------------------------------------------------");


            // println!("E-Graph: {:?}", runner.egraph);
            println!(" Stop Reason: {:?}",runner.stop_reason);
        },
        Err(e) => {
            // Handle the error here
            println!("Failed to parse the graph: {}", e);
        }
    }
}
