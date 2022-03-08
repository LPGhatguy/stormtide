import { createContext, useState } from "react"

export const CombatContext = createContext(null)

export default function CombatState({ children }) {
  const [attackers, setAttackers] = useState([])
  const [blockers, setBlockers] = useState([])

  const value = {
    attackers,
    blockers,
    setAttackers,
    setBlockers,
  }

  return (
    <CombatContext.Provider value={value}>{children}</CombatContext.Provider>
  )
}
