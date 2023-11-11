# Blit

- Learn me a Rust / Bevy / ECS / gamedev
- Make me a roguelike

## Ideas

- setting: post apocalyptic Earth; you play a Goblin, attempting to overthrow their Human slavemasters.
- provision and lead a small band of freedom fighters into their lair to rescue other slaves, fight your way deeper into their fortresses
- optional prequel: Papers Please / Man in the High Castle mashup where you play a bureaucrat in the employ of slaver lordlings, attempting to steal and sequester as many resources for the resistance as you can until you are caught and sentenced to death by public torture
- mechanical focus:
  - torchlight-level resource management
  - highly lethal, simulationist turn-based combat
    - stamina and risk-oriented combat choices
    - time-based turns with interruption mechanics
      - choose when to start an action
      - and when to let an opponent move first - wait for them to tire, make a mistake, or overextend
    - reach & positioning are critical
    - Fight! / sword & scoundrel style situationally useful combat choices and manouevres
    - brutal flanking penalties, morale, etc
    - line of sight is not the only fog of war - orientation in melee is combat expertise
    - no HP, only wounds, dismemberment, shock and blood loss, adrenaline, infection and sepsis
  - control a party of up to 6 directly; extras (eg. freed captives) are uncontrolled, and provide a pool of reinforcements for the dead
  - veterans have improved morale. discipline, and awareness in combat, but do little extra to hit or survive being hit
  - to survive, be the ambusher, not the ambushed; set traps, skulk, outwit and outflank the enemy.
  - goblins are weak toe to toe, but have excellent night vision and a diabolical knack with traps
  - every rescued captive is another mouth to feed  ..
- Dwarf-Fortress style indirect control of non-core party - agents have Thoughts, and pursue goal-oriented actions. Your job is to set certain parameters for official duties, and influence through policy and environment design.
- Highly differentiated, uneven characters
 - trait based
 - wide stat system
 - skills, stats improve through use (success + failure) or through time-based training in downtime

## game system - design notes

core skill test mechanic: roll under

(d12 * 2) + d10

https://anydice.com/program/32d8e

TN = APTITUDE (primary stat + half secondary stat + skill) + ADVANTAGES - DIFFICULTY

--

OPPOSED / Combat checks =  … ? 

-- 

Untrained / beginners luck tests? 

--

STATS (15d10, allocate / random assignment)

- Ethos (will)
- Logos (reason / acuity) Pathos (charisma)
- Nous (intuition)
- Metis (wisdom / cunning)
---
- Thew (power)
- Sinew (speed)
- Ligament (dexterity / fine motor skill)
- Marrow (vitality)
---
- Water (composure)
- Glass (perception / focus)
- Wind (agility / coordination)
---
- Wire (reflex / awareness)
- Wood (endurance)
---
- Dust (§ magic, use artefact, etc)

STAT	RANK
- incapacitated	0   
- impaired	1   
- deficient	2  
- inferior	3  
- poor	4  
- ordinary	5  
- decent	6  
- talented	7    
- gifted	8  
- exceptional	9  
- perfect (human limit)	10 
- preternatural	11 
- mythic	12 
- divine	13+

skills (open at 1):

SKILL	RANK
- amateur	1-2  
- apprentice	3-4  
- journeyman	5-6  
- accomplished	7-8  
- adept	9-10  
- expert	10-11 
- champion / luminary	12-13
- master	13   
- grand master	14   

baseline (50% TN) = 18:

DIFFICULTY	BASELINE	TN/MOD
- foolproof	+18	
- routine	+15	
- easy	+13	
- simple	+10	
- moderate	+7	
- tricky (baseline)	+4	
- hard	+1	
- challenging	-2	
- harrowing	-5	
- desperate	-8	
- impossible	-12	

## Dev Resources
- (https://bevyengine.org/assets/)[bevy learning resources]
- (https://www.youtube.com/playlist?list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd)[learn bevy basics on YT]
- (https://bfnightly.bracketproductions.com/chapter_1.html)[rust roguelike tutorial]
- (https://taintedcoders.com)[bevy gamedev notes]
- (https://bevy-cheatbook.github.io)[bevy cheatsheet]
- (https://github.com/64kramsystem/learn_bevy_ecs_by_ripping_off-code/tree/master)[ripping off a roguelike]
- (https://arewegameyet.rs/#ecosystem)[ecosystem]
- (https://github.com/Trouv/bevy_ecs_ldtk)[LDTK integration]
- (https://github.com/rive-app/rive-bevy)[rive integration]
- (https://github.com/StarArawn/bevy_ecs_tilemap)[ECS tilemap]


## Maybe Laters
- (https://github.com/StarArawn/kayak_ui)[kayak]
- (https://github.com/ggez/aseprite)[aseprite]
- (https://crates.io/crates/big-brain)[big braind AI]

## TODO 

 - [x] initial language / framework bikeshedding
 - [x] initial project bikeshedding
 - [x] naive tileset implementation
 - [x] build a grid using Bevy ECS Tileset
 - [ ] move a little @ player avatar

