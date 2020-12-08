use std::collections::HashSet;

fn main() {
    part1();
    part2();
}

fn part1()
{
    let numbers = input();

    for number in &numbers {
        let remainder = 2020 - number;

        if numbers.contains(&remainder) {
            println!("{} x {} = {}", number, remainder, number * remainder);
            break;
        }
    }
}

fn part2()
{
    let numbers = input();

    for number_a in &numbers {
        for number_b in &numbers {
            if number_a == number_b {
                continue;
            }

            if number_a + number_b > 2020 {
                continue;
            }

            let number_c = 2020 - number_a - number_b;

            if number_c == *number_a || number_c == *number_b {
                continue;
            }
    
            if numbers.contains(&number_c) {
                println!("{} x {} x {} = {}", number_a, number_b, number_c, number_a * number_b * number_c);
                return;
            }
        }
    }
}


fn input() -> HashSet<u32> 
{
    let mut numbers = HashSet::new();

    numbers.insert(1977);
    numbers.insert(1515);
    numbers.insert(1857);
    numbers.insert(1800);
    numbers.insert(1737);
    numbers.insert(1778);
    numbers.insert(1505);
    numbers.insert(1958);
    numbers.insert(1982);
    numbers.insert(1941);
    numbers.insert(1417);
    numbers.insert(1232);
    numbers.insert(1234);
    numbers.insert(2005);
    numbers.insert(1637);
    numbers.insert(1956);
    numbers.insert(1252);
    numbers.insert(1457);
    numbers.insert(1494);
    numbers.insert(1317);
    numbers.insert(1388);
    numbers.insert(1630);
    numbers.insert(1207);
    numbers.insert(1536);
    numbers.insert(1225);
    numbers.insert(1369);
    numbers.insert(1343);
    numbers.insert(1502);
    numbers.insert(1616);
    numbers.insert(1744);
    numbers.insert(1950);
    numbers.insert(1280);
    numbers.insert(1647);
    numbers.insert(1780);
    numbers.insert(1435);
    numbers.insert(1915);
    numbers.insert(1365);
    numbers.insert(1707);
    numbers.insert(1795);
    numbers.insert(1554);
    numbers.insert(1652);
    numbers.insert(539);
    numbers.insert(1892);
    numbers.insert(1546);
    numbers.insert(1908);
    numbers.insert(1629);
    numbers.insert(1836);
    numbers.insert(1805);
    numbers.insert(1395);
    numbers.insert(1360);
    numbers.insert(1487);
    numbers.insert(1739);
    numbers.insert(1884);
    numbers.insert(1427);
    numbers.insert(1615);
    numbers.insert(1470);
    numbers.insert(1922);
    numbers.insert(1753);
    numbers.insert(1632);
    numbers.insert(1968);
    numbers.insert(1429);
    numbers.insert(2008);
    numbers.insert(1124);
    numbers.insert(1441);
    numbers.insert(1384);
    numbers.insert(1955);
    numbers.insert(1815);
    numbers.insert(1741);
    numbers.insert(1331);
    numbers.insert(1442);
    numbers.insert(1988);
    numbers.insert(1788);
    numbers.insert(1585);
    numbers.insert(1794);
    numbers.insert(1217);
    numbers.insert(1434);
    numbers.insert(1751);
    numbers.insert(1240);
    numbers.insert(1284);
    numbers.insert(1883);
    numbers.insert(1711);
    numbers.insert(1376);
    numbers.insert(1638);
    numbers.insert(1932);
    numbers.insert(1979);
    numbers.insert(1769);
    numbers.insert(1597);
    numbers.insert(896);
    numbers.insert(1691);
    numbers.insert(1379);
    numbers.insert(1386);
    numbers.insert(1658);
    numbers.insert(2009);
    numbers.insert(1885);
    numbers.insert(1721);
    numbers.insert(1619);
    numbers.insert(1825);
    numbers.insert(1688);
    numbers.insert(1544);
    numbers.insert(1934);
    numbers.insert(1484);
    numbers.insert(1720);
    numbers.insert(1215);
    numbers.insert(1371);
    numbers.insert(1752);
    numbers.insert(1692);
    numbers.insert(1745);
    numbers.insert(1911);
    numbers.insert(1453);
    numbers.insert(1723);
    numbers.insert(1856);
    numbers.insert(1270);
    numbers.insert(1397);
    numbers.insert(812);
    numbers.insert(1610);
    numbers.insert(1712);
    numbers.insert(1829);
    numbers.insert(1524);
    numbers.insert(1541);
    numbers.insert(1338);
    numbers.insert(1383);
    numbers.insert(1592);
    numbers.insert(2006);
    numbers.insert(1823);
    numbers.insert(1410);
    numbers.insert(1422);
    numbers.insert(1394);
    numbers.insert(1933);
    numbers.insert(1572);
    numbers.insert(1697);
    numbers.insert(1736);
    numbers.insert(2003);
    numbers.insert(1301);
    numbers.insert(1817);
    numbers.insert(1902);
    numbers.insert(1389);
    numbers.insert(1490);
    numbers.insert(1705);
    numbers.insert(1329);
    numbers.insert(1458);
    numbers.insert(1510);
    numbers.insert(1625);
    numbers.insert(1676);
    numbers.insert(1443);
    numbers.insert(1539);
    numbers.insert(1710);
    numbers.insert(24);
    numbers.insert(1586);
    numbers.insert(1948);
    numbers.insert(1994);
    numbers.insert(1975);
    numbers.insert(1974);
    numbers.insert(1237);
    numbers.insert(1419);
    numbers.insert(1748);
    numbers.insert(1589);
    numbers.insert(1821);
    numbers.insert(1462);
    numbers.insert(1792);
    numbers.insert(1381);
    numbers.insert(1400);
    numbers.insert(1222);
    numbers.insert(1602);
    numbers.insert(2001);
    numbers.insert(1976);
    numbers.insert(1700);
    numbers.insert(1626);
    numbers.insert(1966);
    numbers.insert(1548);
    numbers.insert(1593);
    numbers.insert(2010);
    numbers.insert(1149);
    numbers.insert(1372);
    numbers.insert(1224);
    numbers.insert(1675);
    numbers.insert(1271);
    numbers.insert(1896);
    numbers.insert(1983);
    numbers.insert(1299);
    numbers.insert(1528);
    numbers.insert(1631);
    numbers.insert(1804);
    numbers.insert(1562);
    numbers.insert(1754);
    numbers.insert(1566);
    numbers.insert(1473);
    numbers.insert(1980);
    numbers.insert(465);
    numbers.insert(1868);
    numbers.insert(1304);
    numbers.insert(1279);
    numbers.insert(1963);
    numbers.insert(1582);
    numbers.insert(1713);
    numbers.insert(1330);
    numbers.insert(1758);
    numbers.insert(1551);
    numbers.insert(1241);
    numbers.insert(1469);
    numbers.insert(1888);

    return numbers;
}
