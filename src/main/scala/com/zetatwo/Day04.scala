package com.zetatwo

import scala.annotation.tailrec

object Day04 {
  val WHITESPACE = raw"\s+"

  def main(args: Array[String]): Unit = {
    val lines: Seq[String] = io.Source.stdin.getLines.toList

    printf("Result 1: %d\n", validate(lines))
    printf("Result 1: %d\n", validate2(lines))
  }

  def validatepassword(password: String): Boolean = {
    @tailrec
    def loop(remainder: List[String], seen: Set[String]): Boolean = remainder match {
      case current :: _ if seen.contains(current) => false
      case current :: rest => loop(rest, seen + current)
      case _ => true
    }

    loop(password.split(WHITESPACE).toList, Set())
  }

  def validatepassword2(password: String): Boolean = {
    @tailrec
    def loop(remainder: List[String], seen: Set[String]): Boolean = remainder match {
      case current :: _ if seen.contains(current.sorted) => false
      case current :: rest => loop(rest, seen + current.sorted)
      case _ => true
    }

    loop(password.split(WHITESPACE).toList, Set())
  }

  def validate(input: Seq[String]): Int = {
    input.count(validatepassword)
  }

  def validate2(input: Seq[String]): Int = {
    input.count(validatepassword2)
  }

}
