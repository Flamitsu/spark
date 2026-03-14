use crate::SparkError;
/// This function should remove the spark installation in the current ESP partition.
pub fn remove_spark_installation(_confirmation: bool) -> Result<(), SparkError>{
    todo!("Remove the spark installation in the ESP and the NVRAM");
}
