ha:cschem-sheet-v1 {
	ha:obj_indirect.1 {
		li:objects {
			ha:group.1 {
				uuid=hYHZy6b6F0KHAkwrg00AAAEi;
				li:objects {
					ha:group.1 {
						uuid=hYHZy6b6F0KHAkwrg00AAAEj; loclib_name=CP150;
						li:objects {
						}
						ha:attrib {
							footprint={rcy(150.0mil,pol=bar-)}
							li:portmap {
								{P->pcb/pinnum=1}
								{N->pcb/pinnum=2}
							}
						}
					}
					ha:group.2 {
						uuid=hYHZy6b6F0KHAkwrg00AAAEk; loclib_name=wspr_cap_p;
						li:objects {
						}
						ha:attrib {
							footprint=wspr-cap-p.lht
							li:portmap {
								{P->pcb/pinnum=1}
								{N->pcb/pinnum=2}
							}
						}
					}
				}
				ha:attrib {
					ha:purpose = { value=devmap; prio=0; }
				}
			}
		}
	}
	ha:obj_direct.2 {
		uuid=culLnBu46vTwVz8BZPkAAAAC;
		li:objects {
			ha:pen.sheet-decor { shape=round; size=125; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.titlebox-frame { shape=round; size=250; color=#777777; font_height=0; }
			ha:pen.titlebox-fill { shape=round; size=250; color=#bbffbb; font_height=0; }
			ha:pen.titlebox-big { shape=round; size=250; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.titlebox-small { shape=round; size=250; color=#777777; font_height=1500; font_family=sans; }
			ha:pen.wire { shape=round; size=250; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.bus { shape=round; size=1500; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.hub { shape=round; size=3000; color=#6666ff; font_height=3000; font_family=sans; }
			ha:pen.sym-decor { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; }
			ha:pen.sym-primary { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.sym-secondary { shape=round; size=125; color=#33bb33; font_height=3000; font_family=sans; }
			ha:pen.term-decor { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.term-primary { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.term-secondary { shape=round; size=250; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.busterm-decor { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.busterm-primary { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.busterm-secondary { shape=round; size=1500; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.junction { shape=round; size=1000; color=#2222bb; font_height=3000; font_family=sans; }
			ha:group.2 {
				uuid=culLnBu46vTwVz8BZPkAAAAJ; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=204000; y=116000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=culLnBu46vTwVz8BZPkAAAAK; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=culLnBu46vTwVz8BZPkAAAAL; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=wspr-cap.lht
					name=C1
					role=symbol
					value=<n/a>
				}
			}
			ha:group.3 {
				uuid=culLnBu46vTwVz8BZPkAAAAS; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAk;
				x=224000; y=116000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=culLnBu46vTwVz8BZPkAAAAT; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAl;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=N
							role=terminal
						}
					}
					ha:group.2 {
						uuid=culLnBu46vTwVz8BZPkAAAAU; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAm;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=P
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.7 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
					ha:arc.8 { cx=34000; cy=0; r=23000; sang=167.500000; dang=25.000000; stroke=sym-decor; }
					ha:line.9 { x1=6000; y1=-3000; x2=8000; y2=-3000; stroke=sym-decor; }
					ha:line.10 { x1=7000; y1=-4000; x2=7000; y2=-2000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=wspr_cap_p
					footprint=wspr-cap-p.lht
					name=C2
					role=symbol
					value=<n/a>
				}
			}
			ha:group.4 {
				uuid=culLnBu46vTwVz8BZPkAAAAb; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=156000; y=188000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=culLnBu46vTwVz8BZPkAAAAc; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=culLnBu46vTwVz8BZPkAAAAd; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=acy(300)
					name=R1
					role=symbol
					value=<n/a>
				}
			}
			ha:group.5 {
				uuid=culLnBu46vTwVz8BZPkAAAC+; src_uuid=culLnBu46vTwVz8BZPkAAAC7;
				x=96000; y=184000;
				li:objects {
					ha:text.1 { x1=0; y1=7000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=culLnBu46vTwVz8BZPkAAAC/; src_uuid=culLnBu46vTwVz8BZPkAAAC8;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=culLnBu46vTwVz8BZPkAAADA; src_uuid=culLnBu46vTwVz8BZPkAAAC9;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:polygon.4 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=6000; }
							ha:line { x1=0; y1=6000; x2=4000; y2=6000; }
							ha:line { x1=4000; y1=6000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=connector(1,2)
					name=Vin
					role=symbol
				}
			}
			ha:group.6 {
				uuid=culLnBu46vTwVz8BZPkAAADE; src_uuid=culLnBu46vTwVz8BZPkAAAC7;
				x=12000; y=184000; mirx=1;
				li:objects {
					ha:text.1 { x1=0; y1=7000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=culLnBu46vTwVz8BZPkAAADF; src_uuid=culLnBu46vTwVz8BZPkAAAC8;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=culLnBu46vTwVz8BZPkAAADG; src_uuid=culLnBu46vTwVz8BZPkAAAC9;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:polygon.4 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=6000; }
							ha:line { x1=0; y1=6000; x2=4000; y2=6000; }
							ha:line { x1=4000; y1=6000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=connector(1,2)
					name=5V
					role=symbol
				}
			}
			ha:group.7 {
				uuid=hYHZy6b6F0KHAkwrg00AAAAW; src_uuid=Bfb3T54GnibQqLBxCNUAAAAF;
				x=124000; y=148000;
				li:objects {
					ha:text.1 { x1=0; y1=-4000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.2 {
						li:outline {
							ha:line { x1=0; y1=0; x2=0; y2=32000; }
							ha:line { x1=0; y1=32000; x2=24000; y2=32000; }
							ha:line { x1=24000; y1=32000; x2=24000; y2=0; }
							ha:line { x1=24000; y1=0; x2=0; y2=0; }
						}
						stroke=sym-decor;
					}
					ha:group.3 {
						uuid=hYHZy6b6F0KHAkwrg00AAAAX; src_uuid=Bfb3T54GnibQqLBxCNUAAAAB;
						x=24000; y=20000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=SCL
							pinnum=4
							role=terminal
						}
					}
					ha:group.4 {
						uuid=hYHZy6b6F0KHAkwrg00AAAAY; src_uuid=Bfb3T54GnibQqLBxCNUAAAAC;
						x=24000; y=12000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=SDA
							pinnum=2
							role=terminal
						}
					}
					ha:group.5 {
						uuid=hYHZy6b6F0KHAkwrg00AAAAZ; src_uuid=Bfb3T54GnibQqLBxCNUAAAAD;
						x=12000; y=32000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=VCC
							pinnum=1
							role=terminal
						}
					}
					ha:group.6 {
						uuid=hYHZy6b6F0KHAkwrg00AAAAa; src_uuid=Bfb3T54GnibQqLBxCNUAAAAE;
						x=12000; y=0; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=GND
							pinnum=3
							role=terminal
						}
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Erich Heinzle
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-symbol-generator=boxsym-rnd
					footprint=AM2320sensor.lht
					name=AM2320
					role=symbol
				}
			}
			ha:group.9 {
				uuid=hYHZy6b6F0KHAkwrg00AAAAz; src_uuid=cgTMpN8K8sXz+5UdJNQAAAAG;
				x=140000; y=76000; mirx=1;
				li:objects {
					ha:text.1 { x1=-8000; y1=-4000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.2 {
						li:outline {
							ha:line { x1=0; y1=0; x2=0; y2=32000; }
							ha:line { x1=0; y1=32000; x2=24000; y2=32000; }
							ha:line { x1=24000; y1=32000; x2=24000; y2=0; }
							ha:line { x1=24000; y1=0; x2=0; y2=0; }
						}
						stroke=sym-decor;
					}
					ha:group.3 {
						uuid=hYHZy6b6F0KHAkwrg00AAAA0; src_uuid=cgTMpN8K8sXz+5UdJNQAAAAB;
						x=24000; y=20000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=D0
							pinnum=3
							role=terminal
						}
					}
					ha:group.4 {
						uuid=hYHZy6b6F0KHAkwrg00AAAA1; src_uuid=cgTMpN8K8sXz+5UdJNQAAAAC;
						x=24000; y=16000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=CS
							pinnum=4
							role=terminal
						}
					}
					ha:group.5 {
						uuid=hYHZy6b6F0KHAkwrg00AAAA2; src_uuid=cgTMpN8K8sXz+5UdJNQAAAAD;
						x=24000; y=12000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=CLK
							pinnum=5
							role=terminal
						}
					}
					ha:group.6 {
						uuid=hYHZy6b6F0KHAkwrg00AAAA3; src_uuid=cgTMpN8K8sXz+5UdJNQAAAAE;
						x=12000; y=32000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=VCC
							pinnum=2
							role=terminal
						}
					}
					ha:group.7 {
						uuid=hYHZy6b6F0KHAkwrg00AAAA4; src_uuid=cgTMpN8K8sXz+5UdJNQAAAAF;
						x=12000; y=0; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=GND
							pinnum=1
							role=terminal
						}
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Erich Heinzle
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-symbol-generator=boxsym-rnd
					footprint=connector(1,5)
					name=MAX31855
					role=symbol
				}
			}
			ha:group.10 {
				uuid=hYHZy6b6F0KHAkwrg00AAAA+; src_uuid=85I5+iDqnkZdjOfdbKcAAAAF;
				x=188000; y=76000; mirx=1;
				li:objects {
					ha:text.1 { x1=-8000; y1=-4000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.2 {
						li:outline {
							ha:line { x1=0; y1=0; x2=0; y2=32000; }
							ha:line { x1=0; y1=32000; x2=24000; y2=32000; }
							ha:line { x1=24000; y1=32000; x2=24000; y2=0; }
							ha:line { x1=24000; y1=0; x2=0; y2=0; }
						}
						stroke=sym-decor;
					}
					ha:group.3 {
						uuid=hYHZy6b6F0KHAkwrg00AAAA/; src_uuid=85I5+iDqnkZdjOfdbKcAAAAB;
						x=24000; y=20000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=CLK
							pinnum=1
							role=terminal
						}
					}
					ha:group.4 {
						uuid=hYHZy6b6F0KHAkwrg00AAABA; src_uuid=85I5+iDqnkZdjOfdbKcAAAAC;
						x=24000; y=12000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=DIO
							pinnum=2
							role=terminal
						}
					}
					ha:group.5 {
						uuid=hYHZy6b6F0KHAkwrg00AAABB; src_uuid=85I5+iDqnkZdjOfdbKcAAAAD;
						x=12000; y=32000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=VCC
							pinnum=3
							role=terminal
						}
					}
					ha:group.6 {
						uuid=hYHZy6b6F0KHAkwrg00AAABC; src_uuid=85I5+iDqnkZdjOfdbKcAAAAE;
						x=12000; y=0; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=GND
							pinnum=4
							role=terminal
						}
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Erich Heinzle
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-symbol-generator=boxsym-rnd
					footprint=TM1637LEDDISPLAYcompact.lht
					name=TM1637
					role=symbol
				}
			}
			ha:group.11 {
				uuid=hYHZy6b6F0KHAkwrg00AAABi; src_uuid=m3940KZwpIrqkck0SFIAAAAg;
				x=24000; y=88000;
				li:objects {
					ha:text.1 { x1=0; y1=-4000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.2 {
						li:outline {
							ha:line { x1=0; y1=0; x2=0; y2=92000; }
							ha:line { x1=0; y1=92000; x2=56000; y2=92000; }
							ha:line { x1=56000; y1=92000; x2=56000; y2=0; }
							ha:line { x1=56000; y1=0; x2=0; y2=0; }
						}
						stroke=sym-decor;
					}
					ha:group.3 {
						uuid=hYHZy6b6F0KHAkwrg00AAABj; src_uuid=m3940KZwpIrqkck0SFIAAAAB;
						x=0; y=52000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=0; stroke=term-secondary; text=RESET; }
						}
						ha:attrib {
							name=RESET1
							pinnum=3
							role=terminal
						}
					}
					ha:group.4 {
						uuid=hYHZy6b6F0KHAkwrg00AAABk; src_uuid=m3940KZwpIrqkck0SFIAAAAC;
						x=0; y=48000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=AREF
							pinnum=18
							role=terminal
						}
					}
					ha:group.5 {
						uuid=hYHZy6b6F0KHAkwrg00AAABl; src_uuid=m3940KZwpIrqkck0SFIAAAAD;
						x=0; y=44000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=0; stroke=term-secondary; text=RESET; }
						}
						ha:attrib {
							name=RESET2
							pinnum=28
							role=terminal
						}
					}
					ha:group.6 {
						uuid=hYHZy6b6F0KHAkwrg00AAABm; src_uuid=m3940KZwpIrqkck0SFIAAAAE;
						x=56000; y=88000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=0; stroke=term-secondary; text=D0/TX; }
						}
						ha:attrib {
							name=D0
							pinnum=1
							role=terminal
						}
					}
					ha:group.7 {
						uuid=hYHZy6b6F0KHAkwrg00AAABn; src_uuid=m3940KZwpIrqkck0SFIAAAAF;
						x=56000; y=84000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=0; stroke=term-secondary; text=D1/RX; }
						}
						ha:attrib {
							name=D1
							pinnum=2
							role=terminal
						}
					}
					ha:group.8 {
						uuid=hYHZy6b6F0KHAkwrg00AAABo; src_uuid=m3940KZwpIrqkck0SFIAAAAG;
						x=56000; y=80000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=0; stroke=term-secondary; text=D2/INT0; }
						}
						ha:attrib {
							name=D2
							pinnum=5
							role=terminal
						}
					}
					ha:group.9 {
						uuid=hYHZy6b6F0KHAkwrg00AAABp; src_uuid=m3940KZwpIrqkck0SFIAAAAH;
						x=56000; y=76000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=0; stroke=term-secondary; text=D3/INT1; }
						}
						ha:attrib {
							name=D3
							pinnum=6
							role=terminal
						}
					}
					ha:group.10 {
						uuid=hYHZy6b6F0KHAkwrg00AAABq; src_uuid=m3940KZwpIrqkck0SFIAAAAI;
						x=56000; y=72000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=D4
							pinnum=7
							role=terminal
						}
					}
					ha:group.11 {
						uuid=hYHZy6b6F0KHAkwrg00AAABr; src_uuid=m3940KZwpIrqkck0SFIAAAAJ;
						x=56000; y=68000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=D5
							pinnum=8
							role=terminal
						}
					}
					ha:group.12 {
						uuid=hYHZy6b6F0KHAkwrg00AAABs; src_uuid=m3940KZwpIrqkck0SFIAAAAK;
						x=56000; y=64000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=D6
							pinnum=9
							role=terminal
						}
					}
					ha:group.13 {
						uuid=hYHZy6b6F0KHAkwrg00AAABt; src_uuid=m3940KZwpIrqkck0SFIAAAAL;
						x=56000; y=60000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=D7
							pinnum=10
							role=terminal
						}
					}
					ha:group.14 {
						uuid=hYHZy6b6F0KHAkwrg00AAABu; src_uuid=m3940KZwpIrqkck0SFIAAAAM;
						x=56000; y=56000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=D8
							pinnum=11
							role=terminal
						}
					}
					ha:group.15 {
						uuid=hYHZy6b6F0KHAkwrg00AAABv; src_uuid=m3940KZwpIrqkck0SFIAAAAN;
						x=56000; y=52000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=D9
							pinnum=12
							role=terminal
						}
					}
					ha:group.16 {
						uuid=hYHZy6b6F0KHAkwrg00AAABw; src_uuid=m3940KZwpIrqkck0SFIAAAAO;
						x=56000; y=48000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=0; stroke=term-secondary; text=D10/SS; }
						}
						ha:attrib {
							name=D10
							pinnum=13
							role=terminal
						}
					}
					ha:group.17 {
						uuid=hYHZy6b6F0KHAkwrg00AAABx; src_uuid=m3940KZwpIrqkck0SFIAAAAP;
						x=56000; y=44000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=0; stroke=term-secondary; text=D11/MOSI; }
						}
						ha:attrib {
							name=D11
							pinnum=14
							role=terminal
						}
					}
					ha:group.18 {
						uuid=hYHZy6b6F0KHAkwrg00AAABy; src_uuid=m3940KZwpIrqkck0SFIAAAAQ;
						x=56000; y=40000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=0; stroke=term-secondary; text=D12/MISO; }
						}
						ha:attrib {
							name=D12
							pinnum=15
							role=terminal
						}
					}
					ha:group.19 {
						uuid=hYHZy6b6F0KHAkwrg00AAABz; src_uuid=m3940KZwpIrqkck0SFIAAAAR;
						x=56000; y=36000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=0; stroke=term-secondary; text=D13/SCK; }
						}
						ha:attrib {
							name=D13
							pinnum=16
							role=terminal
						}
					}
					ha:group.20 {
						uuid=hYHZy6b6F0KHAkwrg00AAAB0; src_uuid=m3940KZwpIrqkck0SFIAAAAS;
						x=56000; y=32000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=A0
							pinnum=19
							role=terminal
						}
					}
					ha:group.21 {
						uuid=hYHZy6b6F0KHAkwrg00AAAB1; src_uuid=m3940KZwpIrqkck0SFIAAAAT;
						x=56000; y=28000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=A1
							pinnum=20
							role=terminal
						}
					}
					ha:group.22 {
						uuid=hYHZy6b6F0KHAkwrg00AAAB2; src_uuid=m3940KZwpIrqkck0SFIAAAAU;
						x=56000; y=24000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=A2
							pinnum=21
							role=terminal
						}
					}
					ha:group.23 {
						uuid=hYHZy6b6F0KHAkwrg00AAAB3; src_uuid=m3940KZwpIrqkck0SFIAAAAV;
						x=56000; y=20000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=A3
							pinnum=22
							role=terminal
						}
					}
					ha:group.24 {
						uuid=hYHZy6b6F0KHAkwrg00AAAB4; src_uuid=m3940KZwpIrqkck0SFIAAAAX;
						x=56000; y=16000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=A4
							pinnum=23
							role=terminal
						}
					}
					ha:group.25 {
						uuid=hYHZy6b6F0KHAkwrg00AAAB5; src_uuid=m3940KZwpIrqkck0SFIAAAAY;
						x=56000; y=12000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=A5
							pinnum=24
							role=terminal
						}
					}
					ha:group.26 {
						uuid=hYHZy6b6F0KHAkwrg00AAAB6; src_uuid=m3940KZwpIrqkck0SFIAAAAZ;
						x=56000; y=8000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=A6
							pinnum=25
							role=terminal
						}
					}
					ha:group.27 {
						uuid=hYHZy6b6F0KHAkwrg00AAAB7; src_uuid=m3940KZwpIrqkck0SFIAAAAa;
						x=56000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=A7
							pinnum=26
							role=terminal
						}
					}
					ha:group.28 {
						uuid=hYHZy6b6F0KHAkwrg00AAAB8; src_uuid=m3940KZwpIrqkck0SFIAAAAb;
						x=24000; y=92000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=3v3
							pinnum=17
							role=terminal
						}
					}
					ha:group.29 {
						uuid=hYHZy6b6F0KHAkwrg00AAAB9; src_uuid=m3940KZwpIrqkck0SFIAAAAc;
						x=28000; y=92000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=5V
							pinnum=27
							role=terminal
						}
					}
					ha:group.30 {
						uuid=hYHZy6b6F0KHAkwrg00AAAB+; src_uuid=m3940KZwpIrqkck0SFIAAAAd;
						x=32000; y=92000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=VIN
							pinnum=30
							role=terminal
						}
					}
					ha:group.31 {
						uuid=hYHZy6b6F0KHAkwrg00AAAB/; src_uuid=m3940KZwpIrqkck0SFIAAAAe;
						x=24000; y=0; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=0; stroke=term-secondary; text=GND; }
						}
						ha:attrib {
							name=GND1
							pinnum=4
							role=terminal
						}
					}
					ha:group.32 {
						uuid=hYHZy6b6F0KHAkwrg00AAACA; src_uuid=m3940KZwpIrqkck0SFIAAAAf;
						x=32000; y=0; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=0; stroke=term-secondary; text=GND; }
						}
						ha:attrib {
							name=GND2
							pinnum=29
							role=terminal
						}
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Erich Heinzle
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					-symbol-generator=boxsym-rnd
					footprint=nano_every.lht
					name=NANO
					role=symbol
				}
			}
			ha:group.12 {
				uuid=hYHZy6b6F0KHAkwrg00AAACE; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=168000; y=188000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=hYHZy6b6F0KHAkwrg00AAACF; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=hYHZy6b6F0KHAkwrg00AAACG; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=acy(300)
					name=R2
					role=symbol
					value=<n/a>
				}
			}
			ha:group.13 {
				uuid=hYHZy6b6F0KHAkwrg00AAACI;
				li:objects {
					ha:line.1 { x1=56000; y1=84000; x2=56000; y2=80000; stroke=wire; }
					ha:line.2 { x1=56000; y1=80000; x2=48000; y2=80000; stroke=wire; }
					ha:line.4 { x1=48000; y1=76000; x2=48000; y2=84000; stroke=wire; }
					ha:line.5 { x1=48000; y1=80000; x2=48000; y2=80000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.14 {
				li:conn {
					/2/13/1
					/2/11/32/1
				}
			}
			ha:connection.15 {
				li:conn {
					/2/13/4
					/2/11/31/1
				}
			}
			ha:group.16 {
				uuid=hYHZy6b6F0KHAkwrg00AAACN; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=48000; y=76000;
				li:objects {
					ha:group.1 {
						uuid=hYHZy6b6F0KHAkwrg00AAACO; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:connection.17 {
				li:conn {
					/2/16/1/1
					/2/13/4
				}
			}
			ha:group.18 {
				uuid=hYHZy6b6F0KHAkwrg00AAACT; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=16000; y=180000;
				li:objects {
					ha:group.1 {
						uuid=hYHZy6b6F0KHAkwrg00AAACU; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.19 {
				uuid=hYHZy6b6F0KHAkwrg00AAACV; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=92000; y=180000;
				li:objects {
					ha:group.1 {
						uuid=hYHZy6b6F0KHAkwrg00AAACW; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.20 {
				uuid=hYHZy6b6F0KHAkwrg00AAACX;
				li:objects {
					ha:line.1 { x1=16000; y1=188000; x2=52000; y2=188000; stroke=wire; }
					ha:line.2 { x1=52000; y1=188000; x2=52000; y2=184000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.21 {
				li:conn {
					/2/20/1
					/2/6/3/1
				}
			}
			ha:connection.22 {
				li:conn {
					/2/20/2
					/2/11/29/1
				}
			}
			ha:group.23 {
				uuid=hYHZy6b6F0KHAkwrg00AAACY;
				li:objects {
					ha:line.1 { x1=16000; y1=184000; x2=16000; y2=180000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.24 {
				li:conn {
					/2/23/1
					/2/6/2/1
				}
			}
			ha:connection.25 {
				li:conn {
					/2/23/1
					/2/18/1/1
				}
			}
			ha:group.26 {
				uuid=hYHZy6b6F0KHAkwrg00AAACZ;
				li:objects {
					ha:line.1 { x1=56000; y1=184000; x2=56000; y2=188000; stroke=wire; }
					ha:line.2 { x1=56000; y1=188000; x2=92000; y2=188000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.27 {
				li:conn {
					/2/26/1
					/2/11/30/1
				}
			}
			ha:connection.28 {
				li:conn {
					/2/26/2
					/2/5/3/1
				}
			}
			ha:group.29 {
				uuid=hYHZy6b6F0KHAkwrg00AAACa;
				li:objects {
					ha:line.1 { x1=92000; y1=184000; x2=92000; y2=180000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.30 {
				li:conn {
					/2/29/1
					/2/19/1/1
				}
			}
			ha:connection.31 {
				li:conn {
					/2/29/1
					/2/5/2/1
				}
			}
			ha:group.32 {
				uuid=hYHZy6b6F0KHAkwrg00AAACf; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=52000; y=188000;
				li:objects {
					ha:group.1 {
						uuid=hYHZy6b6F0KHAkwrg00AAACg; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.34 {
				uuid=hYHZy6b6F0KHAkwrg00AAACh; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=136000; y=188000;
				li:objects {
					ha:group.1 {
						uuid=hYHZy6b6F0KHAkwrg00AAACi; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.35 {
				uuid=hYHZy6b6F0KHAkwrg00AAACj; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=176000; y=116000;
				li:objects {
					ha:group.1 {
						uuid=hYHZy6b6F0KHAkwrg00AAACk; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.37 {
				uuid=hYHZy6b6F0KHAkwrg00AAAC3;
				x=8000; y=0;
				li:objects {
					ha:line.1 { x1=128000; y1=188000; x2=128000; y2=184000; stroke=wire; }
					ha:line.3 { x1=128000; y1=188000; x2=160000; y2=188000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.40 {
				uuid=hYHZy6b6F0KHAkwrg00AAAC4;
				x=0; y=-72000;
				li:objects {
					ha:line.1 { x1=176000; y1=188000; x2=176000; y2=184000; stroke=wire; }
					ha:line.3 { x1=176000; y1=188000; x2=224000; y2=188000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.43 {
				uuid=hYHZy6b6F0KHAkwrg00AAAC5;
				x=8000; y=0;
				li:objects {
					ha:line.1 { x1=128000; y1=144000; x2=128000; y2=140000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.47 {
				uuid=hYHZy6b6F0KHAkwrg00AAAC7;
				x=8000; y=0;
				li:objects {
					ha:line.1 { x1=148000; y1=168000; x2=144000; y2=168000; stroke=wire; }
				}
				ha:attrib {
					name=SCL
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.48 {
				li:conn {
					/2/47/1
					/2/7/3/1
				}
			}
			ha:group.49 {
				uuid=hYHZy6b6F0KHAkwrg00AAAC8;
				x=8000; y=0;
				li:objects {
					ha:line.2 { x1=144000; y1=160000; x2=160000; y2=160000; stroke=wire; }
					ha:line.3 { x1=160000; y1=160000; x2=160000; y2=168000; stroke=wire; }
				}
				ha:attrib {
					name=SDA
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.50 {
				li:conn {
					/2/7/4/1
					/2/49/2
				}
			}
			ha:group.51 {
				uuid=hYHZy6b6F0KHAkwrg00AAAC9;
				x=68000; y=64000;
				li:objects {
					ha:line.1 { x1=140000; y1=112000; x2=136000; y2=112000; stroke=wire; }
				}
				ha:attrib {
					name=SCL
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.53 {
				uuid=hYHZy6b6F0KHAkwrg00AAAC+;
				x=68000; y=64000;
				li:objects {
					ha:line.1 { x1=140000; y1=104000; x2=136000; y2=104000; stroke=wire; }
				}
				ha:attrib {
					name=SDA
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.55 {
				uuid=hYHZy6b6F0KHAkwrg00AAAC/;
				x=68000; y=64000;
				li:objects {
					ha:line.1 { x1=124000; y1=120000; x2=124000; y2=124000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.57 {
				uuid=hYHZy6b6F0KHAkwrg00AAADA;
				x=68000; y=68000;
				li:objects {
					ha:line.1 { x1=124000; y1=88000; x2=124000; y2=92000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.59 {
				uuid=hYHZy6b6F0KHAkwrg00AAADB;
				x=-28000; y=-12000;
				li:objects {
					ha:line.1 { x1=156000; y1=80000; x2=156000; y2=84000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.61 {
				uuid=hYHZy6b6F0KHAkwrg00AAADG; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=192000; y=156000;
				li:objects {
					ha:group.1 {
						uuid=hYHZy6b6F0KHAkwrg00AAADH; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.63 {
				uuid=hYHZy6b6F0KHAkwrg00AAADI; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=128000; y=68000;
				li:objects {
					ha:group.1 {
						uuid=hYHZy6b6F0KHAkwrg00AAADJ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.65 {
				uuid=hYHZy6b6F0KHAkwrg00AAADK; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=136000; y=140000;
				li:objects {
					ha:group.1 {
						uuid=hYHZy6b6F0KHAkwrg00AAADL; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.67 {
				uuid=hYHZy6b6F0KHAkwrg00AAADM; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=176000; y=68000;
				li:objects {
					ha:group.1 {
						uuid=hYHZy6b6F0KHAkwrg00AAADN; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.70 {
				uuid=hYHZy6b6F0KHAkwrg00AAADS; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=192000; y=188000;
				li:objects {
					ha:group.1 {
						uuid=hYHZy6b6F0KHAkwrg00AAADT; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.72 {
				uuid=hYHZy6b6F0KHAkwrg00AAADU; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=128000; y=116000;
				li:objects {
					ha:group.1 {
						uuid=hYHZy6b6F0KHAkwrg00AAADV; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.73 {
				uuid=hYHZy6b6F0KHAkwrg00AAADW;
				x=-28000; y=-12000;
				li:objects {
					ha:line.1 { x1=156000; y1=124000; x2=156000; y2=128000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.90 {
				li:conn {
					/2/59/1
					/2/9/7/1
				}
			}
			ha:connection.91 {
				li:conn {
					/2/63/1/1
					/2/59/1
				}
			}
			ha:connection.92 {
				li:conn {
					/2/73/1
					/2/9/6/1
				}
			}
			ha:connection.93 {
				li:conn {
					/2/73/1
					/2/72/1/1
				}
			}
			ha:group.94 {
				uuid=hYHZy6b6F0KHAkwrg00AAADa; src_uuid=hYHZy6b6F0KHAkwrg00AAAC/;
				x=104000; y=64000;
				li:objects {
					ha:line.1 { x1=124000; y1=120000; x2=124000; y2=124000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.96 {
				uuid=hYHZy6b6F0KHAkwrg00AAADb; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=228000; y=188000;
				li:objects {
					ha:group.1 {
						uuid=hYHZy6b6F0KHAkwrg00AAADc; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.98 {
				uuid=hYHZy6b6F0KHAkwrg00AAADf; src_uuid=hYHZy6b6F0KHAkwrg00AAADd;
				x=248000; y=172000;
				li:objects {
					ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=wire; }
				}
				ha:attrib {
					name=SCL
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.100 {
				uuid=hYHZy6b6F0KHAkwrg00AAADg; src_uuid=hYHZy6b6F0KHAkwrg00AAADe;
				x=248000; y=172000;
				li:objects {
					ha:line.1 { x1=0; y1=4000; x2=-4000; y2=4000; stroke=wire; }
				}
				ha:attrib {
					name=SDA
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.103 {
				uuid=hYHZy6b6F0KHAkwrg00AAADk; src_uuid=hYHZy6b6F0KHAkwrg00AAADA;
				x=104000; y=52000;
				li:objects {
					ha:line.1 { x1=124000; y1=88000; x2=124000; y2=92000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.105 {
				uuid=hYHZy6b6F0KHAkwrg00AAADl; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=228000; y=140000;
				li:objects {
					ha:group.1 {
						uuid=hYHZy6b6F0KHAkwrg00AAADm; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.107 {
				uuid=hYHZy6b6F0KHAkwrg00AAADp; src_uuid=hYHZy6b6F0KHAkwrg00AAADo;
				x=248000; y=156000;
				li:objects {
					ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=wire; }
				}
				ha:attrib {
					name=MOSI
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.108 {
				uuid=hYHZy6b6F0KHAkwrg00AAADq; src_uuid=hYHZy6b6F0KHAkwrg00AAADn;
				x=248000; y=156000;
				li:objects {
					ha:line.1 { x1=0; y1=-4000; x2=-4000; y2=-4000; stroke=wire; }
				}
				ha:attrib {
					name=SS
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.109 {
				uuid=hYHZy6b6F0KHAkwrg00AAADr; src_uuid=hYHZy6b6F0KHAkwrg00AAADo;
				x=248000; y=168000;
				li:objects {
					ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=wire; }
				}
				ha:attrib {
					name=SCK
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.111 {
				uuid=hYHZy6b6F0KHAkwrg00AAADs; src_uuid=hYHZy6b6F0KHAkwrg00AAADn;
				x=248000; y=168000;
				li:objects {
					ha:line.1 { x1=0; y1=-8000; x2=-4000; y2=-8000; stroke=wire; }
				}
				ha:attrib {
					name=MISO
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.117 {
				uuid=hYHZy6b6F0KHAkwrg00AAADt; src_uuid=hYHZy6b6F0KHAkwrg00AAADo;
				x=88000; y=136000;
				li:objects {
					ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=wire; }
				}
				ha:attrib {
					name=SS
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.118 {
				li:conn {
					/2/117/1
					/2/11/16/1
				}
			}
			ha:group.119 {
				uuid=hYHZy6b6F0KHAkwrg00AAADu; src_uuid=hYHZy6b6F0KHAkwrg00AAADn;
				x=88000; y=136000;
				li:objects {
					ha:line.1 { x1=0; y1=-4000; x2=-4000; y2=-4000; stroke=wire; }
				}
				ha:attrib {
					name=MOSI
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.120 {
				li:conn {
					/2/119/1
					/2/11/17/1
				}
			}
			ha:group.125 {
				uuid=hYHZy6b6F0KHAkwrg00AAADx; src_uuid=hYHZy6b6F0KHAkwrg00AAADo;
				x=88000; y=104000;
				li:objects {
					ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=wire; }
				}
				ha:attrib {
					name=SDA
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.126 {
				li:conn {
					/2/125/1
					/2/11/24/1
				}
			}
			ha:group.127 {
				uuid=hYHZy6b6F0KHAkwrg00AAADy; src_uuid=hYHZy6b6F0KHAkwrg00AAADn;
				x=88000; y=104000;
				li:objects {
					ha:line.1 { x1=0; y1=-4000; x2=-4000; y2=-4000; stroke=wire; }
				}
				ha:attrib {
					name=SCL
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.128 {
				li:conn {
					/2/127/1
					/2/11/25/1
				}
			}
			ha:connection.129 {
				li:conn {
					/2/40/1
					/2/10/5/1
				}
			}
			ha:connection.130 {
				li:conn {
					/2/40/1
					/2/35/1/1
					/2/40/3
				}
			}
			ha:group.135 {
				uuid=hYHZy6b6F0KHAkwrg00AAAD0; src_uuid=hYHZy6b6F0KHAkwrg00AAADn;
				x=160000; y=96000;
				li:objects {
					ha:line.2 { x1=-8000; y1=-8000; x2=0; y2=-8000; stroke=wire; }
					ha:line.3 { x1=-8000; y1=-8000; x2=-8000; y2=28000; stroke=wire; }
					ha:line.4 { x1=-8000; y1=28000; x2=-48000; y2=28000; stroke=wire; }
					ha:line.5 { x1=-48000; y1=28000; x2=-48000; y2=68000; stroke=wire; }
					ha:line.6 { x1=-48000; y1=68000; x2=-76000; y2=68000; stroke=wire; }
				}
				ha:attrib {
					name=D3
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.136 {
				li:conn {
					/2/10/4/1
					/2/135/2
				}
			}
			ha:connection.138 {
				li:conn {
					/2/9/4/1
					/2/147/4
				}
			}
			ha:group.144 {
				uuid=hYHZy6b6F0KHAkwrg00AAAD5;
				li:objects {
					ha:line.1 { x1=108000; y1=128000; x2=88000; y2=128000; stroke=wire; }
					ha:line.2 { x1=112000; y1=96000; x2=108000; y2=96000; stroke=wire; }
					ha:line.3 { x1=108000; y1=96000; x2=108000; y2=128000; stroke=wire; }
					ha:line.4 { x1=88000; y1=128000; x2=84000; y2=128000; stroke=wire; }
				}
				ha:attrib {
					name=MISO
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.145 {
				li:conn {
					/2/144/2
					/2/9/3/1
				}
			}
			ha:connection.146 {
				li:conn {
					/2/144/4
					/2/11/18/1
				}
			}
			ha:group.147 {
				uuid=hYHZy6b6F0KHAkwrg00AAAD6;
				li:objects {
					ha:line.2 { x1=104000; y1=140000; x2=104000; y2=92000; stroke=wire; }
					ha:line.4 { x1=104000; y1=92000; x2=112000; y2=92000; stroke=wire; }
					ha:line.5 { x1=84000; y1=140000; x2=104000; y2=140000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.148 {
				li:conn {
					/2/147/5
					/2/11/15/1
				}
			}
			ha:group.149 {
				uuid=hYHZy6b6F0KHAkwrg00AAAD7;
				li:objects {
					ha:line.1 { x1=88000; y1=124000; x2=84000; y2=124000; stroke=wire; }
					ha:line.2 { x1=100000; y1=124000; x2=88000; y2=124000; stroke=wire; }
					ha:line.3 { x1=100000; y1=88000; x2=112000; y2=88000; stroke=wire; }
					ha:line.4 { x1=100000; y1=88000; x2=100000; y2=124000; stroke=wire; }
				}
				ha:attrib {
					name=SCK
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.150 {
				li:conn {
					/2/149/1
					/2/11/19/1
				}
			}
			ha:connection.151 {
				li:conn {
					/2/149/3
					/2/9/5/1
				}
			}
			ha:group.154 {
				uuid=hYHZy6b6F0KHAkwrg00AAAD9;
				li:objects {
					ha:line.1 { x1=84000; y1=168000; x2=116000; y2=168000; stroke=wire; }
					ha:line.2 { x1=116000; y1=128000; x2=156000; y2=128000; stroke=wire; }
					ha:line.3 { x1=160000; y1=96000; x2=156000; y2=96000; stroke=wire; }
					ha:line.4 { x1=156000; y1=128000; x2=156000; y2=96000; stroke=wire; }
					ha:line.5 { x1=116000; y1=168000; x2=116000; y2=128000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.155 {
				li:conn {
					/2/154/1
					/2/11/8/1
				}
			}
			ha:connection.156 {
				li:conn {
					/2/154/3
					/2/10/3/1
				}
			}
			ha:connection.157 {
				li:conn {
					/2/135/6
					/2/11/9/1
				}
			}
			ha:connection.162 {
				li:conn {
					/2/37/1
					/2/7/5/1
				}
			}
			ha:connection.163 {
				li:conn {
					/2/37/1
					/2/34/1/1
					/2/37/3
				}
			}
			ha:connection.164 {
				li:conn {
					/2/43/1
					/2/7/6/1
				}
			}
			ha:connection.165 {
				li:conn {
					/2/65/1/1
					/2/43/1
				}
			}
			ha:connection.170 {
				li:conn {
					/2/4/1/1
					/2/47/1
				}
			}
			ha:connection.174 {
				li:conn {
					/2/70/1/1
					/2/55/1
				}
			}
			ha:connection.176 {
				li:conn {
					/2/96/1/1
					/2/94/1
				}
			}
			ha:connection.178 {
				li:conn {
					/2/105/1/1
					/2/103/1
				}
			}
			ha:connection.180 {
				li:conn {
					/2/4/2/1
					/2/37/3
				}
			}
			ha:connection.182 {
				li:conn {
					/2/12/2/1
					/2/37/3
				}
			}
			ha:connection.183 {
				li:conn {
					/2/49/3
					/2/12/1/1
				}
			}
			ha:connection.184 {
				li:conn {
					/2/2/2/1
					/2/40/3
				}
			}
			ha:connection.185 {
				li:conn {
					/2/40/3
					/2/3/2/1
				}
			}
			ha:group.191 {
				uuid=hYHZy6b6F0KHAkwrg00AAAD/;
				li:objects {
					ha:line.1 { x1=224000; y1=68000; x2=224000; y2=96000; stroke=wire; }
					ha:line.2 { x1=176000; y1=68000; x2=176000; y2=72000; stroke=wire; }
					ha:line.3 { x1=176000; y1=68000; x2=224000; y2=68000; stroke=wire; }
					ha:line.4 { x1=204000; y1=68000; x2=204000; y2=96000; stroke=wire; }
					ha:line.5 { x1=204000; y1=68000; x2=204000; y2=68000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.192 {
				li:conn {
					/2/191/1
					/2/3/1/1
				}
			}
			ha:connection.193 {
				li:conn {
					/2/191/2
					/2/10/6/1
				}
			}
			ha:connection.194 {
				li:conn {
					/2/191/2
					/2/67/1/1
					/2/191/3
				}
			}
			ha:connection.195 {
				li:conn {
					/2/191/4
					/2/2/1/1
				}
			}
			ha:connection.196 {
				li:conn {
					/2/32/1/1
					/2/20/1
					/2/20/2
				}
			}
			ha:group.197 {
				uuid=hYHZy6b6F0KHAkwrg00AAAEJ; src_uuid=Br9GzYwRmeINS+F4qQ8AAAAJ;
				x=216000; y=148000;
				li:objects {
					ha:text.1 { x1=-8000; y1=-4000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.2 {
						li:outline {
							ha:line { x1=0; y1=0; x2=0; y2=32000; }
							ha:line { x1=0; y1=32000; x2=24000; y2=32000; }
							ha:line { x1=24000; y1=32000; x2=24000; y2=0; }
							ha:line { x1=24000; y1=0; x2=0; y2=0; }
						}
						stroke=sym-decor;
					}
					ha:group.3 {
						uuid=hYHZy6b6F0KHAkwrg00AAAEK; src_uuid=Br9GzYwRmeINS+F4qQ8AAAAB;
						x=24000; y=28000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=SDA
							pinnum=3
							role=terminal
						}
					}
					ha:group.4 {
						uuid=hYHZy6b6F0KHAkwrg00AAAEL; src_uuid=Br9GzYwRmeINS+F4qQ8AAAAC;
						x=24000; y=24000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=SCL
							pinnum=4
							role=terminal
						}
					}
					ha:group.5 {
						uuid=hYHZy6b6F0KHAkwrg00AAAEM; src_uuid=Br9GzYwRmeINS+F4qQ8AAAAD;
						x=24000; y=20000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=SCK
							pinnum=5
							role=terminal
						}
					}
					ha:group.6 {
						uuid=hYHZy6b6F0KHAkwrg00AAAEN; src_uuid=Br9GzYwRmeINS+F4qQ8AAAAE;
						x=24000; y=12000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=MISO
							pinnum=6
							role=terminal
						}
					}
					ha:group.7 {
						uuid=hYHZy6b6F0KHAkwrg00AAAEO; src_uuid=Br9GzYwRmeINS+F4qQ8AAAAF;
						x=24000; y=8000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=MOSI
							pinnum=7
							role=terminal
						}
					}
					ha:group.8 {
						uuid=hYHZy6b6F0KHAkwrg00AAAEP; src_uuid=Br9GzYwRmeINS+F4qQ8AAAAG;
						x=24000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=SS
							pinnum=8
							role=terminal
						}
					}
					ha:group.9 {
						uuid=hYHZy6b6F0KHAkwrg00AAAEQ; src_uuid=Br9GzYwRmeINS+F4qQ8AAAAH;
						x=12000; y=32000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=VCC
							pinnum=1
							role=terminal
						}
					}
					ha:group.10 {
						uuid=hYHZy6b6F0KHAkwrg00AAAER; src_uuid=Br9GzYwRmeINS+F4qQ8AAAAI;
						x=12000; y=0; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=GND
							pinnum=2
							role=terminal
						}
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Erich Heinzle
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-symbol-generator=boxsym-rnd
					footprint=DeekRobot8122DataLoggingRTCBoard.lht
					name=DeekSDRTC
					role=symbol
				}
			}
			ha:connection.198 {
				li:conn {
					/2/197/3/1
					/2/100/1
				}
			}
			ha:connection.199 {
				li:conn {
					/2/197/4/1
					/2/98/1
				}
			}
			ha:connection.200 {
				li:conn {
					/2/197/5/1
					/2/109/1
				}
			}
			ha:connection.201 {
				li:conn {
					/2/197/6/1
					/2/111/1
				}
			}
			ha:connection.202 {
				li:conn {
					/2/197/7/1
					/2/107/1
				}
			}
			ha:connection.203 {
				li:conn {
					/2/197/8/1
					/2/108/1
				}
			}
			ha:connection.204 {
				li:conn {
					/2/197/9/1
					/2/94/1
				}
			}
			ha:connection.205 {
				li:conn {
					/2/197/10/1
					/2/103/1
				}
			}
			ha:group.206 {
				uuid=hYHZy6b6F0KHAkwrg00AAAEa; src_uuid=T6JsD51qiopVec3mawgAAAAI;
				x=184000; y=164000;
				li:objects {
					ha:text.1 { x1=-4000; y1=-4000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.2 {
						li:outline {
							ha:line { x1=0; y1=0; x2=0; y2=16000; }
							ha:line { x1=0; y1=16000; x2=16000; y2=16000; }
							ha:line { x1=16000; y1=16000; x2=16000; y2=0; }
							ha:line { x1=16000; y1=0; x2=0; y2=0; }
						}
						stroke=sym-decor;
					}
					ha:group.3 {
						uuid=hYHZy6b6F0KHAkwrg00AAAEb; src_uuid=T6JsD51qiopVec3mawgAAAAB;
						x=0; y=12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=WAK
							pinnum=2
							role=terminal
						}
					}
					ha:group.4 {
						uuid=hYHZy6b6F0KHAkwrg00AAAEc; src_uuid=T6JsD51qiopVec3mawgAAAAC;
						x=0; y=8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=INT
							pinnum=3
							role=terminal
						}
					}
					ha:group.5 {
						uuid=hYHZy6b6F0KHAkwrg00AAAEd; src_uuid=T6JsD51qiopVec3mawgAAAAD;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=RST
							pinnum=1
							role=terminal
						}
					}
					ha:group.6 {
						uuid=hYHZy6b6F0KHAkwrg00AAAEe; src_uuid=T6JsD51qiopVec3mawgAAAAE;
						x=16000; y=12000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=SCL
							pinnum=5
							role=terminal
						}
					}
					ha:group.7 {
						uuid=hYHZy6b6F0KHAkwrg00AAAEf; src_uuid=T6JsD51qiopVec3mawgAAAAF;
						x=16000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=SDA
							pinnum=4
							role=terminal
						}
					}
					ha:group.8 {
						uuid=hYHZy6b6F0KHAkwrg00AAAEg; src_uuid=T6JsD51qiopVec3mawgAAAAG;
						x=8000; y=16000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=VCC
							pinnum=7
							role=terminal
						}
					}
					ha:group.9 {
						uuid=hYHZy6b6F0KHAkwrg00AAAEh; src_uuid=T6JsD51qiopVec3mawgAAAAH;
						x=8000; y=0; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=GND
							pinnum=6
							role=terminal
						}
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Erich Heinzle
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-symbol-generator=boxsym-rnd
					footprint=DFRobotCCS811Breakout.lht
					name=CCS811
					role=symbol
				}
			}
			ha:connection.209 {
				li:conn {
					/2/206/7/1
					/2/53/1
				}
			}
			ha:connection.210 {
				li:conn {
					/2/206/8/1
					/2/55/1
				}
			}
			ha:connection.211 {
				li:conn {
					/2/51/1
					/2/206/6/1
				}
			}
			ha:connection.212 {
				li:conn {
					/2/57/1
					/2/206/9/1
				}
			}
			ha:connection.213 {
				li:conn {
					/2/61/1/1
					/2/57/1
				}
			}
			ha:text.214 { x1=88000; y1=104000; dyntext=0; stroke=sheet-decor; text=SDA; }
			ha:text.215 { x1=88000; y1=100000; dyntext=0; stroke=sheet-decor; text=SCL; }
		}
		ha:attrib {
			drawing_min_height=200000
			drawing_min_width=287000
			maintainer=<maint. attr>
			page=<page attr>
			print_page=A/4
			title=<please set sheet title attribute>
		}
	}
  li:sch-rnd-conf-v1 {
   ha:overwrite {
    ha:editor {
     grids_idx = 2
     grid = 4.0960 mm
    }
   }
  }
}
