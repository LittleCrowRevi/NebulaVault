using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

[CreateAssetMenu(menuName = "Variables/Health")]
public class Health : ScriptableObject
{
    [SerializeField] public int baseHealth;
    [SerializeField] public int currentHealth;
}