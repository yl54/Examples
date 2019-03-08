package scala

import java.time.{ Instant, LocalDate, ZoneOffset }
import java.time.format.DateTimeFormatter

object BasicScala {
    def SeqToString(list: Seq[String], delimiter: String): String = {
        list.mkString(delimiter)
    }

    val DateHourFormat: DateTimeFormatter = DateTimeFormatter ofPattern "yyyy-MM-dd HH"
    val dateFormat: DateTimeFormatter     = DateTimeFormatter.ofPattern("yyyy-MM-dd")

    def getPriorDaysList(numPriorDays: Int,
                         processDateStr: String): List[String] = {
        // Get the calendar instance of the process date
        val processDate = LocalDate.parse(processDateStr, dateFormat)

        val priorDaysList = (0 to numPriorDays).map(processDate.minusDays(_)).toList
        priorDaysList.map(dt => {
            getDateString(dt)
        })
    }

    def getDateString(processDate: LocalDate): String = {
        processDate.format(dateFormat)
    }
}