package com.zetatwo

import org.scalatest.FunSuite

class Day03Tests extends FunSuite {
  test("Day03.layerparams") {
    assert(Day03.layerparams(1) == (0, 1, 1))
    assert(Day03.layerparams(8) == (1, 2, 9))
    assert(Day03.layerparams(23) == (2, 10, 25))
  }
}