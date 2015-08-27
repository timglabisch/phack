extern crate semver;
extern crate cargo;
#[macro_use]
mod easy_resolver;
use cargo::core::{Dependency, Summary};
use std::collections::HashMap;
use easy_resolver::*;

fn main() {

    // registry beinhaltet eine liste aller Packages und dessen Versionen.
    // Möchte man diese Liste aufbauen, so muss man nach und nach alle Package-Informationen von Packagist runterladen
    // siehe: https://packagist.org/p/symfony/symfony.json
    // und die entsprechenden Versionen eintragen.
    // Die Registry beinhaltet alle verfügbaren Pakete (welche benötigt werden könnten).
    let mut registry = vec!(
        pkg!("symfony", "2.7.0"),
        pkg!("symfony", "2.7.1"),
        // ...
        pkg!("foo", "1.0.0"),
        pkg!("foo", "2.0.0"),
        pkg!("foo", "0.1.0"),
        pkg!("foo", "0.2.0"),
        pkg!("food", "0.2.0"),
        pkg!("symfony/finder", "2.7.0"),
        
        // der Paket Syntax erlaubt das definieren von Abhängigkeiten.
        // jede Abhängigkeit (dep(endency)) ist ein Paket, welches in der Registry vorliegen muss.
        pkg!("symfony", "2.7.3" => [dep("symfony/finder", "2.7.0")]),
        pkg!("d1", "0.4.2" => [dep("foo", "1"), dep("food", "0.2.*")]),
        pkg!("d2", "0.4.2" => [dep("foo", "2")]),
        pkg!("d3", "0.4.2" => [dep("foo", "0.1")]),
        pkg!("d4", "0.4.2" => [dep("foo", "0.2")]),

        // Das eigentliche Projekt (eigener Paketname) muss auch in die Registry gelegt werden,
        // sowie die Abhängigkeiten. Die Abhängigkeiten hier, ist quasi die Liste aus einer composer.lock.
        pkg!("base_project", "1.0.0" => [dep("symfony", "*")]),
    );

    // nun wollen wir wissen, welche Pakete unser Projekt benötigt,
    // wenn alles gut geht, dann wird uns das Programm mitteilen, dass wir symfony und symfony/finder benötigen.
    let packages = vec![
        dep("base_project", "*"),
    ];

    let res = resolve(packages, &mut registry).unwrap();

    for package in res {
        // hier könnte man via git die pakete einfach runterladen,
        // der resolver hat rausgefunden welche pakete und vorallem welche versionen benötigt werden.
        println!("{} {}", package.name(), package.version()); // symfony, symfony-finder
    }
}
