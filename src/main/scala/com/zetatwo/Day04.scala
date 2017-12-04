package com.zetatwo

object Day04 {
  def main(args: Array[String]): Unit = {
    val lines: Seq[String] = io.Source.stdin.getLines.toList

    printf("Result 1: %d\n", validate(lines))
    printf("Result 1: %d\n", validate2(lines))
  }

  def validatepassword(password: String): Boolean = {
    def loop(remainder: Seq[String], seen: Set[String]): Boolean = {
      if (remainder.isEmpty)
       true
      else if (seen.contains(remainder.head))
        false
      else
        loop(remainder.tail, seen + remainder.head)
    }

    loop(password.split("\\s+"), Set())
  }

  def validatepassword2(password: String): Boolean = {
    def loop(remainder: Seq[String], seen: Set[String]): Boolean = {
      if (remainder.isEmpty)
        true
      else if (seen.contains(remainder.head.sorted))
        false
      else
        loop(remainder.tail, seen + remainder.head.sorted)
    }

    loop(password.split("\\s+"), Set())
  }

  def validate(input: Seq[String]): Int = {
    input.count(validatepassword)
  }

  def validate2(input: Seq[String]): Int = {
    input.count(validatepassword2)
  }

}