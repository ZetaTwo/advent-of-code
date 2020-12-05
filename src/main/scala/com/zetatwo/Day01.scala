package com.zetatwo

object Day01 {
  def main(args: Array[String]): Unit = {
    val input = io.StdIn.readLine()

    printf("Result 1: %d", sum(input))
    printf("Result 2: %d", sum2(input))
  }

  def doSum(input: String, rotation: Int): Int = {
    val digits: Seq[Int] = input.map(c => Integer.valueOf(c.toString).intValue())

    val (left, right) = digits.splitAt(rotation)
    val rotated: Seq[Int] = right ++ left

    val pairs: Seq[(Int, Int)] = digits zip rotated
    val pairvalues: Seq[Int] = pairs.flatMap({
      case (a, b) if a == b => Some(a)
      case _ => None
    })

    pairvalues.sum
  }

  def sum(input: String): Int = {
    doSum(input, 1)
  }

  def sum2(input: String): Int = {
    doSum(input, input.length/2)
  }

}
