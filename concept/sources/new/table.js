const tabela = document.getElementById("tab01");

const formatDec = (arr) => {
  if (!Array.isArray(arr)) return arr ?? "";
  return arr
    .map((row) => {
      const copy = [...row];
      const last = copy.length - 1;
      if (last >= 0 && Array.isArray(copy[last])) {
        const [frNum = "", frDen = ""] = copy[last];
        copy[last] = `<sup>${frNum}</sup>&frasl;<sub>${frDen}</sub>`;
      }
      return `<span class="nw">${copy.join("\u200A")}</span>`;
    })
    .join("<br>");
};

const formatBin = (arr) => {
  if (!Array.isArray(arr)) return arr ?? "";
  return arr
    .map((row) => {
      const copy = [...row];
      const last = copy.length - 1;
      if (last >= 2) {
        copy[last] = `<u>${copy[last]}</u>`;
      }
      return `<span class="nw">${copy.join("\u200A")}<sub>2</sub></span>`;
    })
    .join("<br>");
};

const parseCell = (val) => {
  const isObj = typeof val === "object" && val !== null;
  const hasEgo3 = isObj && val.ego3 !== undefined;
  const isSplit = hasEgo3 && val.ego2 !== val.ego3;
  return {
    isSplit,
    first: isObj ? (val.ego2 ?? "") : (val ?? ""),
    second: isSplit ? (val.ego3 ?? "") : "",
  };
};

const getSosaFields = (s, legacyBin = undefined) => {
  // Nowy format: { ego2: { dec: [...], bin: [...] }, ego3: { dec: [...], bin: [...] } }
  if (typeof s === "object" && s !== null && ("ego2" in s || "ego3" in s)) {
    const isNewNested =
      (typeof s.ego2 === "object" && s.ego2 !== null) ||
      (typeof s.ego3 === "object" && s.ego3 !== null);

    if (isNewNested) {
      const decObj = {
        ego2: formatDec(s.ego2?.dec),
        ...(s.ego3 !== undefined ? { ego3: formatDec(s.ego3?.dec) } : {}),
      };
      const binObj = {
        ego2: formatBin(s.ego2?.bin),
        ...(s.ego3 !== undefined ? { ego3: formatBin(s.ego3?.bin) } : {}),
      };
      return {
        dec: parseCell(decObj),
        bin: parseCell(binObj),
      };
    }
  }

  // Wariant bez podziału na ego2/ego3: { dec: [...], bin: [...] }
  if (typeof s === "object" && s !== null && ("dec" in s || "bin" in s)) {
    return {
      dec: parseCell(formatDec(s.dec)),
      bin: parseCell(formatBin(s.bin)),
    };
  }

  // Kompatybilność wsteczna ze starym zapisem (osobne sosa i sbin)
  return {
    dec: parseCell(s),
    bin: parseCell(legacyBin),
  };
};

dane.forEach((sekcja) => {
  const tbody = document.createElement("tbody");
  tbody.innerHTML = `<tr><td class="ac dystans-naglowek" x="RING" colspan="14">${sekcja.dystans}</td></tr>`;

  sekcja.grupy.forEach((grupa) => {
    const totalGroupRows = grupa.relacje.reduce((sum, rel) => {
      const { dec: sosaDec, bin: sosaBin } = getSosaFields(rel.sosa, rel.sbin);
      const { dec: sosaRelDec, bin: sosaRelBin } = getSosaFields(rel.sosa_rel);
      const hasSplit = [
        sosaDec,
        sosaBin,
        sosaRelDec,
        sosaRelBin,
        parseCell(rel.pl),
        parseCell(rel.la),
        parseCell(rel.en),
      ].some((cell) => cell.isSplit);
      return sum + (hasSplit ? 2 : 1);
    }, 0);

    grupa.relacje.forEach((rel, index) => {
      const { dec: sosaDec, bin: sosaBin } = getSosaFields(rel.sosa, rel.sbin);
      const { dec: sosaRelDec, bin: sosaRelBin } = getSosaFields(rel.sosa_rel);
      const pl = parseCell(rel.pl);
      const la = parseCell(rel.la);
      const en = parseCell(rel.en);

      const isAnySplit =
        sosaDec.isSplit ||
        sosaBin.isSplit ||
        sosaRelDec.isSplit ||
        sosaRelBin.isSplit ||
        pl.isSplit ||
        la.isSplit ||
        en.isSplit;
      const relSpan = isAnySplit ? 2 : 1;

      const isFemale = rel.sex?.startsWith("♀");
      const sexSuffix = isFemale ? "_WW" : "";
      const sosaEgo2Suffix = isFemale ? "_WM" : "";
      const sosaEgo3Suffix = isFemale ? "_WW" : "_MW";

      const tr = document.createElement("tr");
      let html = "";

      if (index === 0) {
        html += `<td rowspan="${totalGroupRows}">${grupa.r || ""}</td>`;
        html += `<td rowspan="${totalGroupRows}"><b>rp:</b>${grupa.rp}<hr/><b>sp:</b>${grupa.sp}<hr/><b>spg:<br/>${grupa.spg || ""}</td>`;
        html += `<td rowspan="${totalGroupRows}" x="TIER">${grupa.pokolenie ?? ""}</td>`;
        html += `<td rowspan="${totalGroupRows}" x="RING">${grupa.otoczenie ?? ""}</td>`;
        html += `<td rowspan="${totalGroupRows}" x="KNOT_HEAP">${grupa.symbol}</td>`;
      }

      html += `
        <td rowspan="${relSpan}" x="SEX${sexSuffix}">${rel.sex ?? "-"}</td>
        <td rowspan="${relSpan}" x="KNOT_ATOM${sexSuffix}">${rel.knot}</td>
        <td${sosaDec.isSplit ? "" : ` rowspan="${relSpan}"`} x="SOSA_DEC${sosaEgo2Suffix}">${sosaDec.first}</td>
        <td${sosaBin.isSplit ? "" : ` rowspan="${relSpan}"`} x="SOSA_BIN${sosaEgo2Suffix}">${sosaBin.first}</td>
        <td${sosaRelDec.isSplit ? "" : ` rowspan="${relSpan}"`} x="SOSA_REL_DEC${sosaEgo2Suffix}">⦗${sosaRelDec.first}⦘</td>
      `;
		
        //<td${sosaRelBin.isSplit ? "" : ` rowspan="${relSpan}"`} x="SOSA_REL_BIN${sosaEgo2Suffix}">${sosaRelBin.first}</td>

		html += `
        <td${pl.isSplit ? "" : ` rowspan="${relSpan}"`}>${pl.first}</td>
        <td${la.isSplit ? "" : ` rowspan="${relSpan}"`}>${la.first}</td>
        <td${en.isSplit ? "" : ` rowspan="${relSpan}"`}>${en.first}</td>
      `;

      if (index === 0) {
        html += `<td rowspan="${totalGroupRows}">${grupa.r || ""}</td>`;
      }

      tr.innerHTML = html;
      tbody.appendChild(tr);

      if (isAnySplit) {
        const tr2 = document.createElement("tr");
        let tr2Html = "";
        if (sosaDec.isSplit) tr2Html += `<td x="SOSA_DEC${sosaEgo3Suffix}">${sosaDec.second}</td>`;
        if (sosaBin.isSplit) tr2Html += `<td x="SOSA_BIN${sosaEgo3Suffix}">${sosaBin.second}</td>`;
        if (sosaRelDec.isSplit) tr2Html += `<td x="SOSA_REL_DEC${sosaEgo3Suffix}">⦗${sosaRelDec.second}⦘</td>`;
       // if (sosaRelBin.isSplit) tr2Html += `<td x="SOSA_REL_BIN${sosaEgo3Suffix}">${sosaRelBin.second}</td>`;
        if (pl.isSplit) tr2Html += `<td>${pl.second}</td>`;
        if (la.isSplit) tr2Html += `<td>${la.second}</td>`;
        if (en.isSplit) tr2Html += `<td>${en.second}</td>`;
        tr2.innerHTML = tr2Html;
        tbody.appendChild(tr2);
      }
    });
  });

  tabela.appendChild(tbody);
});