package com.zetatwo

object Day03 {
  /*
  def main(args: Array[String]): Unit = {
    for {
      line: String <- io.Source.stdin.getLines
      input: Int <- Integer.valueOf(line.trim).intValue()
      result: Int <- distance(input)
    } yield printf("Result 1: %d", result)
    //printf("Result 2: %d", linesums2(lines))
  }*/

  def layerparams(tileindex: Int): (Int, Int, Int) = {
    val layerindex = Stream.from(0)
    val layersize = layerindex.map(idx => (1+2*idx)*(1+2*idx))
    tileindex match {
      case 1 => (0, 1, 1)
      case _ => (for {
        (idx, (size, nextsize)) <- layerindex zip (layersize zip layersize.tail)
        if tileindex < nextsize
      } yield (idx + 1, size + 1, nextsize)).head
    }
  }

  def distance(input: Int): Int = {
    1
  }
}
