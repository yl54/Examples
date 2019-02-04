package scala

object BasicScala {
    def SeqToString(list: Seq[String], delimiter: String): String = {
        list.mkString(delimiter)
    }
}