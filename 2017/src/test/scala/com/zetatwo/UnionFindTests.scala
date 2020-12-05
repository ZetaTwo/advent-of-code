package com.zetatwo

import org.scalatest.FunSuite


class UnionFindTests extends FunSuite {
  test("UnionFindTests.HashUnionFind") {
    val step0 = HashUnionFind.create(6)
    val step1 = step0.union(0, 1)
    val step2 = step1.union(0, 2)

    val step3 = step2.union(3, 4)
    val step4 = step3.union(3, 5)

    val step5 = step4.union(1, 4)

    assert(step5.connected(2, 3))
  }
}
