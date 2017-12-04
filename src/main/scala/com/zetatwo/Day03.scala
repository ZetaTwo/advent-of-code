package com.zetatwo

object Day03 {
  type Coordinate = (Int, Int)

  def main(args: Array[String]): Unit = {
    val input: Int = io.StdIn.readLine().trim.toInt

    printf("Result 1: %d", distance(input))
    printf("Result 2: %d", sums(input))
  }

  def layerparams(tileindex: Int): (Int, Int) = {
    tileindex match {
      case 1 => (1, 1)
      case _ =>
        val layerapprox = math.sqrt(tileindex).toInt
        val sidelength = if (layerapprox % 2 == 1) layerapprox else layerapprox - 1
        (sidelength + 2, sidelength * sidelength + 1)
    }
  }

  def sidedistance(start: Int, sidelength: Int): Int = {
    math.abs((sidelength-1)/2) + math.abs(start - (sidelength-1)/2)
  }

  def distance(tileindex: Int, sidelength: Int, startidx: Int): Int = {
    val offset = tileindex - startidx + 1
    val sideoffset = sidelength - 1
    sidedistance(offset % sideoffset, sidelength)
  }

  def distance(input: Int): Int = {
    input match {
      case 1 => 0
      case _ =>
        val (sidelength, startidx) = layerparams(input)
        distance(input, sidelength, startidx)
    }
  }

  def nextcoord(coord: Coordinate, sidelength: Int): Coordinate = {
    val r = (sidelength-1)/2
    coord match {
      case (0, 0) => (1, 0)
      case (x, y) if (x == y) && (y == r) => (x-1, y) // Top right
      case (x, y) if (-x == y) && (y == r) => (x, y - 1) // Top left
      case (x, y) if (-x == -y) && (-y == r) => (x + 1, y) // Bottom left
      case (x, y) if (x == -y) && (-y == r) => (x + 1, y) // Bottom right

      case (x, y) if x == r => (x, y + 1) // Right side
      case (x, y) if -x == r => (x, y - 1) // Left side
      case (x, y) if y == r => (x - 1, y) // Top side
      case (x, y) if -y == r => (x + 1, y) // Bottom side
    }
  }

  def nextval(newcoord: Coordinate, cells: Map[Coordinate, Int]): Int = {
    val (x, y) = newcoord
    cells.getOrElse(  (x + 1, y    ), 0) +
      cells.getOrElse((x + 1, y + 1), 0) +
      cells.getOrElse((x    , y + 1), 0) +
      cells.getOrElse((x - 1, y + 1), 0) +
      cells.getOrElse((x - 1, y    ), 0) +
      cells.getOrElse((x - 1, y - 1), 0) +
      cells.getOrElse((x    , y - 1), 0) +
      cells.getOrElse((x + 1, y - 1), 0)
  }

  def nextsidelength(newcoord: Coordinate, sidelength: Int): Int = {
    val (x, y) = newcoord
    if (x > 0 && x+y-1 == 0) 2*x+1 else sidelength
  }

  def sums(input: Int): Int = {
    def loop(limit: Int, cells: Map[Coordinate, Int], coord: Coordinate, sidelength: Int): Int = {
      val newcoord = nextcoord(coord, sidelength)
      val newval = nextval(newcoord, cells)
      val newsidelength = nextsidelength(newcoord, sidelength)

      if (newval > limit)
        newval
      else
        loop(limit, cells + (newcoord -> newval), newcoord, newsidelength)

    }

    loop(input, Map((0,0) -> 1), (0,0), 1)
  }
}
