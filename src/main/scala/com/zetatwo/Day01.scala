package com.zetatwo

object Day01 {
  def main(args: Array[String]): Unit = {
    val input = io.StdIn.readLine()

    printf("Result 1: %d", sum(input))
    printf("Result 2: %d", sum2(input))
  }

  def doSum(input: String, rotation: Int): Int = {
    val digits: Seq[Int] = input.map(c => Integer.valueOf(c.toString).intValue())
    val rotated: Seq[Int] = digits.slice(rotation, digits.length) ++ digits.slice(0, rotation)
    val pairs: Seq[(Int, Int)] = digits zip rotated
    val pairvalues: Seq[Int] = pairs.flatMap(p => if (p._1 == p._2) List(p._1) else List())
    pairvalues.sum
  }

  def sum(input: String): Int = {
    doSum(input, 1)
  }

  def sum2(input: String): Int = {
    doSum(input, input.length/2)
  }

}
