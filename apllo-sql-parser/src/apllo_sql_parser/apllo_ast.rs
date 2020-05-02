/// The AST of APLLO SQL.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum AplloAST {
    /// DROP TABLE ...
    DropTable {
        /// Table to DROP
        table_name: String,
    },
}
